#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use crate::api::{compare_commits, list_members};
use crate::error::ThxContribError;
use clap::{FromArgMatches, IntoApp, Parser};
use dotenv::dotenv;
use napi::bindgen_prelude::{Error as NapiError, Result, Status};
use regex::Regex;
use std::env;

mod api;
mod error;

#[napi]
async fn run(args: Vec<String>) -> Result<()> {
  dotenv().ok();
  let app = Cli::command();
  let matches = app.get_matches_from(args);
  let cli = Cli::from_arg_matches(&matches).map_err(|e| ThxContribError::cli_error::<Cli>(e))?;
  let should_include_org_members = match cli.include_org_members {
    Some(v) => v,
    None => false,
  };
  let parsed_excludes = match cli.excludes {
    Some(e) => e,
    None => vec!["renovate[bot]".to_string(), "renovate-bot".into()],
  };

  dbg!(&parsed_excludes);

  let gh_token = env::var("GITHUB_ACCESS_TOKEN").map_err(|e| ThxContribError::from(e))?;

  let commits = compare_commits(&cli.owner, cli.repo, cli.base, cli.head, &gh_token).await?;
  let org_members = list_members(&cli.owner, &gh_token).await?;

  if commits.commits.is_empty() {
    return Err(NapiError::new(
      Status::InvalidArg,
      "Couldn't find any relevant commits. Are you sure you used the correct head & base?"
        .to_owned(),
    ));
  }

  let pr_regex = Regex::new(r"(.*)\(#([0-9]+)\)").unwrap();

  let entries: Vec<Entry> = commits
    .commits
    .into_iter()
    .map(|c| {
      let first_line = c.commit.message.lines().next().map_or("", |f| f);

      let msg_and_pr = match pr_regex.captures(first_line) {
        Some(caps) => {
          let msg = caps
            .get(1)
            .map_or(None, |m| Some(m.as_str().trim_end().to_string()));
          let pr = caps.get(2).map_or(None, |m| Some(m.as_str().to_string()));
          (msg, pr)
        }
        None => (None, None),
      };

      let author = match &c.author {
        Some(author) => author.login.to_owned(),
        None => c.commit.author.name,
      };
      let author_url = match &c.author {
        Some(author) => Some(author.html_url.to_owned()),
        None => None,
      };

      Entry {
        author,
        author_url,
        message: msg_and_pr.0,
        pr_number: msg_and_pr.1,
      }
    })
    .filter(|i| {
      if should_include_org_members {
        true
      } else {
        let excludes: Vec<&String> = parsed_excludes.iter().chain(&org_members).collect();
        dbg!(excludes);
        false
      }
    })
    .collect();

  dbg!(org_members);

  Ok(())
}

#[derive(Parser)]
#[clap(
  author = "LekoArts",
  name = "@lekoarts/thanks-contributors",
  about = "This little script accesses GitHub's API to get all contributors and their PRs between two distinct points in the history of commits. This is helpful for changelogs where you'd want to list all contributions for that release (so e.g. changes between v1 and v1.1)."
)]
#[clap(no_binary_name = true)]
struct Cli {
  /// Pointer from where to start looking for changes
  #[clap(required = true)]
  base: String,
  /// Pointer until where to stop looking for changes
  #[clap(required = true)]
  head: String,
  /// Name of the owner/user/organization of the repository
  #[clap(default_value = "gatsbyjs")]
  owner: String,
  /// Name of the repository
  #[clap(default_value = "gatsby")]
  repo: String,
  /// Whether to include organization members into the list or not [default: false]
  #[clap(short, long)]
  include_org_members: Option<bool>,
  /// List of members to exclude from the list. Usage: -e=member1,member2 [default: "renovate-bot", "renovate[bot]"]
  #[clap(
    short,
    long,
    multiple_values = true,
    takes_value = true,
    use_value_delimiter = true,
    require_value_delimiter = true
  )]
  excludes: Option<Vec<String>>,
}

#[derive(Debug)]
struct Entry {
  author: String,
  author_url: Option<String>,
  message: Option<String>,
  pr_number: Option<String>,
}
