use chrono::{
  format::{DelayedFormat, StrftimeItems},
  DateTime, Utc,
};
use std::collections::HashMap;

pub fn get_current_date<'a>() -> DelayedFormat<StrftimeItems<'a>> {
  let now: DateTime<Utc> = Utc::now();
  now.format("%Y-%m-%d_%H-%M-%S")
}

pub fn group_by_author(input: Vec<Entry>) -> HashMap<String, Vec<Entry>> {
  let mut groups: HashMap<String, Vec<Entry>> = HashMap::new();

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

  format!(" [PR #{number}](https://github.com/{owner}/{repo}/pull/{number})")
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

  fn get_foo_entry() -> Entry {
    Entry {
      author: String::from("foo"),
      author_url: None,
      message: Some(String::from("One")),
      pr_number: None,
    }
  }

  fn get_foo2_entry() -> Entry {
    Entry {
      author: String::from("foo"),
      author_url: None,
      message: Some(String::from("Two")),
      pr_number: None,
    }
  }

  fn get_bar_entry() -> Entry {
    Entry {
      author: String::from("bar"),
      author_url: None,
      message: Some(String::from("Three")),
      pr_number: None,
    }
  }

  #[test]
  fn group_by_author_correct() {
    let input: Vec<Entry> = vec![get_foo_entry(), get_foo2_entry(), get_bar_entry()];

    let mut assert = HashMap::new();
    assert.insert(String::from("foo"), vec![get_foo_entry(), get_foo2_entry()]);
    assert.insert(String::from("bar"), vec![get_bar_entry()]);

    assert_eq!(group_by_author(input), assert);
  }

  #[test]
  fn group_by_author_empty_input() {
    assert_eq!(group_by_author(vec![]), HashMap::new())
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
      String::from(" [PR #123](https://github.com/owner/repo/pull/123)")
    )
  }
}
