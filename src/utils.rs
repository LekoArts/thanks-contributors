use chrono::{
  format::{DelayedFormat, StrftimeItems},
  DateTime, Utc,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;

use crate::api::Commit;

pub fn get_current_date<'a>() -> DelayedFormat<StrftimeItems<'a>> {
  let now: DateTime<Utc> = Utc::now();
  now.format("%Y-%m-%d_%H-%M-%S")
}

pub fn group_by_author(input: Vec<Entry>) -> BTreeMap<String, Vec<Entry>> {
  // Use a BTreeMap since its keys are sorted alphabetically
  let mut groups: BTreeMap<String, Vec<Entry>> = BTreeMap::new();

  for e in input {
    let group = groups.entry(e.author.clone()).or_default();
    group.push(e);
  }

  groups
}

pub fn get_pr_link(entry: &Entry, owner: &str, repo: &str) -> String {
  let Some(number) = &entry.pr_number else {
    return String::from("");
  };

  format!("[PR #{number}](https://github.com/{owner}/{repo}/pull/{number})")
}

pub fn parse_msg_and_pr(input: &str) -> MsgAndPr {
  lazy_static! {
    // Regex is not dynamic so .unwrap is fine
    static ref RE: Regex = Regex::new(r"^(?P<msg>.*)\(#(?P<pr>[0-9]+)\)").unwrap();
  }

  match RE.captures(input) {
    Some(caps) => {
      let message = caps.name("msg").map(|m| m.as_str().trim_end().to_string());
      let pr_number = caps.name("pr").map(|m| m.as_str().to_string());

      MsgAndPr { message, pr_number }
    }
    None => MsgAndPr {
      message: None,
      pr_number: None,
    },
  }
}

pub fn create_entries(
  commits: Vec<Commit>,
  should_include_org_members: bool,
  parsed_excludes: Vec<String>,
  org_members: Vec<String>,
) -> Vec<Entry> {
  commits
    .into_iter()
    .map(|c| {
      let first_line = c.commit.message.lines().next().map_or("", |f| f);
      let msg_and_pr = parse_msg_and_pr(first_line);

      let mut author = c.commit.author.name;
      let mut author_url: Option<String> = None;

      if let Some(a) = c.author {
        author = a.login;
        author_url = Some(a.html_url);
      };

      Entry {
        author,
        author_url,
        message: msg_and_pr.message,
        pr_number: msg_and_pr.pr_number,
      }
    })
    .filter(|i| {
      if should_include_org_members {
        true
      } else {
        // Exclude members from the final list of entries
        !parsed_excludes
          .iter()
          .chain(&org_members)
          .any(|x| x == &i.author)
      }
    })
    .collect()
}

pub fn create_output(groups: BTreeMap<String, Vec<Entry>>, owner: &str, repo: &str) -> String {
  let mut output = String::new();

  for (author_name, author_entries) in groups {
    let md_author = match &author_entries[0].author_url {
      Some(url) => format!("[{author_name}]({url})"),
      None => author_name,
    };

    if author_entries.len() > 1 {
      let mut md_author_list = String::new();
      for entry in author_entries {
        if let Some(msg) = &entry.message {
          let line = format!("  - {} {}\n", msg, get_pr_link(&entry, owner, repo));
          md_author_list.push_str(&line)
        };
      }

      let text = format!("- {md_author}\n{md_author_list}");

      output.push_str(&text);
    } else {
      let pr_link = get_pr_link(&author_entries[0], owner, repo);

      if let Some(msg) = &author_entries[0].message {
        let text = format!("- {md_author}: {msg} {pr_link}\n");
        output.push_str(&text);
      };
    }
  }

  output
}

#[derive(Eq, PartialEq, Debug)]
pub struct MsgAndPr {
  pub message: Option<String>,
  pub pr_number: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entry {
  pub author: String,
  pub author_url: Option<String>,
  pub message: Option<String>,
  pub pr_number: Option<String>,
}

#[cfg(test)]
mod tests {
  use crate::api::{CommitAuthor, CommitMeta, CommitMetaAuthor};

  use super::*;

  fn entry_a_one() -> Entry {
    Entry {
      author: "author-a".to_string(),
      author_url: Some("author-a-url".to_string()),
      message: Some("fix(scope): Message".to_string()),
      pr_number: Some("1".to_string()),
    }
  }

  fn entry_a_two() -> Entry {
    Entry {
      author: "author-a".to_string(),
      author_url: Some("author-a-url".to_string()),
      message: Some("fix(scope): Message".to_string()),
      pr_number: Some("2".to_string()),
    }
  }

  fn entry_b() -> Entry {
    Entry {
      author: "author-b".to_string(),
      author_url: Some("author-b-url".to_string()),
      message: Some("fix(scope): Message".to_string()),
      pr_number: Some("3".to_string()),
    }
  }

  fn entry_c() -> Entry {
    Entry {
      author: "author-c".to_string(),
      author_url: None,
      message: Some("fix(scope): Message".to_string()),
      pr_number: Some("4".to_string()),
    }
  }

  #[test]
  fn group_by_author_correct() {
    let input: Vec<Entry> = vec![entry_a_one(), entry_a_two(), entry_b()];

    let mut assert = BTreeMap::new();
    assert.insert("author-a".to_string(), vec![entry_a_one(), entry_a_two()]);
    assert.insert("author-b".to_string(), vec![entry_b()]);

    assert_eq!(group_by_author(input), assert)
  }

  #[test]
  fn group_by_author_empty_input() {
    assert_eq!(group_by_author(vec![]), BTreeMap::new())
  }

  #[test]
  fn pr_link_no_number() {
    assert_eq!(
      get_pr_link(
        &Entry {
          author: "foo".to_string(),
          author_url: None,
          message: None,
          pr_number: None,
        },
        "owner",
        "repo"
      ),
      "".to_string()
    )
  }

  #[test]
  fn pr_link_correct() {
    assert_eq!(
      get_pr_link(
        &Entry {
          author: "foo".to_string(),
          author_url: None,
          message: None,
          pr_number: Some("123".to_string()),
        },
        "owner",
        "repo"
      ),
      "[PR #123](https://github.com/owner/repo/pull/123)".to_string()
    )
  }

  #[test]
  fn parse_msg_and_pr_no_result() {
    assert_eq!(
      parse_msg_and_pr("(((test)))"),
      MsgAndPr {
        message: None,
        pr_number: None
      }
    )
  }

  #[test]
  fn parse_msg_and_pr_correct() {
    assert_eq!(
      parse_msg_and_pr("fix(scope): Message (#123)"),
      MsgAndPr {
        message: Some("fix(scope): Message".to_string()),
        pr_number: Some("123".to_string())
      }
    )
  }

  #[test]
  fn parse_msg_and_pr_correct_backport() {
    assert_eq!(
      parse_msg_and_pr("fix(scope): Message (#123) (#456)"),
      MsgAndPr {
        message: Some("fix(scope): Message (#123)".to_string()),
        pr_number: Some("456".to_string())
      }
    )
  }

  #[test]
  fn parse_msg_and_pr_only_msg() {
    assert_eq!(
      parse_msg_and_pr("fix(scope): Message"),
      MsgAndPr {
        message: None,
        pr_number: None
      }
    )
  }

  fn get_org_members() -> Vec<String> {
    vec!["author-d".to_string()]
  }

  fn get_excludes() -> Vec<String> {
    vec!["author-e".to_string()]
  }

  fn commits_data() -> Vec<Commit> {
    vec![
      Commit {
        url: "url-1".to_string(),
        commit: CommitMeta {
          url: "url-1".to_string(),
          message: "fix(scope): Message (#1)".to_string(),
          author: CommitMetaAuthor {
            name: "author-a".to_string(),
          },
        },
        author: Some(CommitAuthor {
          login: "author-a".to_string(),
          html_url: "author-a-url".to_string(),
        }),
      },
      Commit {
        url: "url-2".to_string(),
        commit: CommitMeta {
          url: "url-2".to_string(),
          message: "fix(scope): Message (#2)".to_string(),
          author: CommitMetaAuthor {
            name: "author-a".to_string(),
          },
        },
        author: Some(CommitAuthor {
          login: "author-a".to_string(),
          html_url: "author-a-url".to_string(),
        }),
      },
      Commit {
        url: "url-3".to_string(),
        commit: CommitMeta {
          url: "url-3".to_string(),
          message: "fix(scope): Message (#3)".to_string(),
          author: CommitMetaAuthor {
            name: "author-b".to_string(),
          },
        },
        author: Some(CommitAuthor {
          login: "author-b".to_string(),
          html_url: "author-b-url".to_string(),
        }),
      },
      Commit {
        url: "url-4".to_string(),
        commit: CommitMeta {
          url: "url-4".to_string(),
          message: "fix(scope): Message (#4)".to_string(),
          author: CommitMetaAuthor {
            name: "author-c".to_string(),
          },
        },
        author: None,
      },
      Commit {
        url: "url-5".to_string(),
        commit: CommitMeta {
          url: "url-5".to_string(),
          message: "fix(scope): Message (#5)".to_string(),
          author: CommitMetaAuthor {
            name: "author-d".to_string(),
          },
        },
        author: None,
      },
      Commit {
        url: "url-6".to_string(),
        commit: CommitMeta {
          url: "url-6".to_string(),
          message: "fix(scope): Message (#6)".to_string(),
          author: CommitMetaAuthor {
            name: "author-e".to_string(),
          },
        },
        author: None,
      },
    ]
  }

  #[test]
  fn create_entries_defaults() {
    assert_eq!(
      create_entries(commits_data(), false, vec![], get_org_members()),
      vec![
        Entry {
          author: "author-a".to_string(),
          author_url: Some("author-a-url".to_string()),
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("1".to_string())
        },
        Entry {
          author: "author-a".to_string(),
          author_url: Some("author-a-url".to_string()),
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("2".to_string())
        },
        Entry {
          author: "author-b".to_string(),
          author_url: Some("author-b-url".to_string()),
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("3".to_string())
        },
        Entry {
          author: "author-c".to_string(),
          author_url: None,
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("4".to_string())
        },
        Entry {
          author: "author-e".to_string(),
          author_url: None,
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("6".to_string())
        }
      ]
    )
  }

  #[test]
  fn create_entries_include_org_members() {
    assert_eq!(
      create_entries(commits_data(), true, vec![], get_org_members()),
      vec![
        Entry {
          author: "author-a".to_string(),
          author_url: Some("author-a-url".to_string()),
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("1".to_string())
        },
        Entry {
          author: "author-a".to_string(),
          author_url: Some("author-a-url".to_string()),
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("2".to_string())
        },
        Entry {
          author: "author-b".to_string(),
          author_url: Some("author-b-url".to_string()),
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("3".to_string())
        },
        Entry {
          author: "author-c".to_string(),
          author_url: None,
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("4".to_string())
        },
        Entry {
          author: "author-d".to_string(),
          author_url: None,
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("5".to_string())
        },
        Entry {
          author: "author-e".to_string(),
          author_url: None,
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("6".to_string())
        }
      ]
    )
  }

  #[test]
  fn create_entries_defaults_with_excludes() {
    assert_eq!(
      create_entries(commits_data(), false, get_excludes(), get_org_members()),
      vec![
        Entry {
          author: "author-a".to_string(),
          author_url: Some("author-a-url".to_string()),
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("1".to_string())
        },
        Entry {
          author: "author-a".to_string(),
          author_url: Some("author-a-url".to_string()),
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("2".to_string())
        },
        Entry {
          author: "author-b".to_string(),
          author_url: Some("author-b-url".to_string()),
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("3".to_string())
        },
        Entry {
          author: "author-c".to_string(),
          author_url: None,
          message: Some("fix(scope): Message".to_string()),
          pr_number: Some("4".to_string())
        }
      ]
    )
  }

  #[test]
  fn create_output_single_entry() {
    let mut groups = BTreeMap::new();
    groups.insert("author-b".to_string(), vec![entry_b()]);

    assert_eq!(create_output(groups, "owner", "repo"), "- [author-b](author-b-url): fix(scope): Message [PR #3](https://github.com/owner/repo/pull/3)\n".to_string())
  }

  #[test]
  fn create_output_multiple_entries() {
    let mut groups = BTreeMap::new();
    groups.insert("author-a".to_string(), vec![entry_a_one(), entry_a_two()]);

    assert_eq!(create_output(groups, "owner", "repo"), "- [author-a](author-a-url)\n  - fix(scope): Message [PR #1](https://github.com/owner/repo/pull/1)\n  - fix(scope): Message [PR #2](https://github.com/owner/repo/pull/2)\n".to_string())
  }

  #[test]
  fn create_output_no_author_url() {
    let mut groups = BTreeMap::new();
    groups.insert("author-c".to_string(), vec![entry_c()]);

    assert_eq!(
      create_output(groups, "owner", "repo"),
      "- author-c: fix(scope): Message [PR #4](https://github.com/owner/repo/pull/4)\n".to_string()
    )
  }

  #[test]
  fn create_output_no_author_url_multiple_entries() {
    let mut groups = BTreeMap::new();
    groups.insert("author-c".to_string(), vec![entry_c(), entry_c()]);

    assert_eq!(create_output(groups, "owner", "repo"), "- author-c\n  - fix(scope): Message [PR #4](https://github.com/owner/repo/pull/4)\n  - fix(scope): Message [PR #4](https://github.com/owner/repo/pull/4)\n".to_string())
  }
}
