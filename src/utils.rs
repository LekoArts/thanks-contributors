use chrono::{
  format::{DelayedFormat, StrftimeItems},
  DateTime, Utc,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;

pub fn get_current_date<'a>() -> DelayedFormat<StrftimeItems<'a>> {
  let now: DateTime<Utc> = Utc::now();
  now.format("%Y-%m-%d_%H-%M-%S")
}

pub fn group_by_author(input: Vec<Entry>) -> BTreeMap<String, Vec<Entry>> {
  // Use a BTreeMap since its keys are sorted alphabetically
  let mut groups: BTreeMap<String, Vec<Entry>> = BTreeMap::new();

  for e in input {
    let group = groups.entry(e.author.clone()).or_insert(vec![]);
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

#[derive(PartialEq, Debug)]
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
  use super::*;

  fn foo_entry() -> Entry {
    Entry {
      author: String::from("foo"),
      author_url: None,
      message: Some(String::from("One")),
      pr_number: None,
    }
  }

  fn foo2_entry() -> Entry {
    Entry {
      author: String::from("foo"),
      author_url: None,
      message: Some(String::from("Two")),
      pr_number: None,
    }
  }

  fn bar_entry() -> Entry {
    Entry {
      author: String::from("bar"),
      author_url: None,
      message: Some(String::from("Three")),
      pr_number: None,
    }
  }

  #[test]
  fn group_by_author_correct() {
    let input: Vec<Entry> = vec![foo_entry(), foo2_entry(), bar_entry()];

    let mut assert = BTreeMap::new();
    assert.insert(String::from("foo"), vec![foo_entry(), foo2_entry()]);
    assert.insert(String::from("bar"), vec![bar_entry()]);

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
          author: String::from("foo"),
          author_url: None,
          message: None,
          pr_number: None,
        },
        "owner",
        "repo"
      ),
      String::from("")
    )
  }

  #[test]
  fn pr_link_correct() {
    assert_eq!(
      get_pr_link(
        &Entry {
          author: String::from("foo"),
          author_url: None,
          message: None,
          pr_number: Some(String::from("123")),
        },
        "owner",
        "repo"
      ),
      String::from("[PR #123](https://github.com/owner/repo/pull/123)")
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
        message: Some(String::from("fix(scope): Message")),
        pr_number: Some(String::from("123"))
      }
    )
  }

  #[test]
  fn parse_msg_and_pr_correct_backport() {
    assert_eq!(
      parse_msg_and_pr("fix(scope): Message (#123) (#456)"),
      MsgAndPr {
        message: Some(String::from("fix(scope): Message (#123)")),
        pr_number: Some(String::from("456"))
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
}
