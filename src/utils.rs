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

  input.into_iter().for_each(|entry| {
    let group = groups.entry(entry.author.clone()).or_insert_with(Vec::new);
    group.push(entry);
  });

  groups
}

pub fn get_pr_link(entry: &Entry, owner: &str, repo: &str) -> String {
  match &entry.pr_number {
    Some(number) => format!(
      " [PR #{}](https://github.com/{}/{}/pull/{})",
      number, owner, repo, number
    ),
    None => String::from(""),
  }
}

#[derive(Debug)]
pub struct Entry {
  pub author: String,
  pub author_url: Option<String>,
  pub message: Option<String>,
  pub pr_number: Option<String>,
}
