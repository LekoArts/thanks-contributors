#[macro_use]
extern crate napi_derive;

use crate::api::{compare_commits, list_members};
use crate::error::ThxContribError;
use crate::utils::{get_current_date, get_pr_link, group_by_author, parse_msg_and_pr, Entry};
use clap::{CommandFactory, FromArgMatches, Parser};
use clap_verbosity_flag::Verbosity;
use dotenv::dotenv;
use log::{debug, info};
use napi::bindgen_prelude::{Error as NapiError, Result, Status};
use std::env;
use std::fs;

pub mod api;
pub mod error;
pub mod utils;

#[allow(dead_code)]
#[napi]
async fn run(args: Vec<String>) -> Result<()> {
  dotenv().ok();
  let app = Cli::command();

  // Arguments are coming from bin.js
  let matches = app.get_matches_from(args);
  let cli = Cli::from_arg_matches(&matches).map_err(ThxContribError::cli_error::<Cli>)?;

  env_logger::Builder::new()
    .filter_level(cli.verbose.log_level_filter())
    .init();

  // By default, don't include org members
  let should_include_org_members = cli.include_org_members.unwrap_or(false);
  // By default, exclude renovate bot
  let parsed_excludes = match cli.excludes {
    Some(e) => e,
    None => vec!["renovate[bot]".to_string(), "renovate-bot".into()],
  };

  debug!("Parsed Excludes: {:#?}", parsed_excludes);

  let gh_token = env::var("GITHUB_ACCESS_TOKEN").map_err(ThxContribError::from)?;

  let commits = compare_commits(&cli.owner, &cli.repo, cli.base, cli.head, &gh_token).await?;
  let org_members = list_members(&cli.owner, &gh_token).await?;

  debug!("Commits: {:#?}", commits);
  debug!("Org members: {:#?}", org_members);

  if commits.commits.is_empty() {
    return Err(NapiError::new(
      Status::InvalidArg,
      "Couldn't find any relevant commits. Are you sure you used the correct head & base?"
        .to_owned(),
    ));
  }

  info!("Fetched {} commits", commits.commits.len());

  let entries: Vec<Entry> = commits
    .commits
    .into_iter()
    .map(|c| {
      let first_line = c.commit.message.lines().next().map_or("", |f| f);
      let msg_and_pr = parse_msg_and_pr(first_line);

      let author = match &c.author {
        Some(author) => author.login.to_owned(),
        None => c.commit.author.name,
      };
      let author_url = c.author.as_ref().map(|author| author.html_url.to_owned());

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
    .collect();

  info!("Process {} filtered commits", entries.len());

  let groups = group_by_author(entries);

  let mut output = String::new();

  for (author_name, author_entries) in groups {
    let md_author = match &author_entries[0].author_url {
      Some(url) => format!("[{author_name}]({url})"),
      None => author_name,
    };

    if author_entries.len() > 1 {
      let mut md_author_list = String::new();
      for entry in author_entries {
        match &entry.message {
          Some(msg) => {
            let line = format!(
              "  - {}{}\n",
              msg,
              get_pr_link(&entry, &cli.owner, &cli.repo)
            );
            md_author_list.push_str(&line)
          }
          None => md_author_list.push_str(""),
        }
      }

      let text = format!("- {md_author}\n{md_author_list}");

      output.push_str(&text);
    } else {
      let pr_link = get_pr_link(&author_entries[0], &cli.owner, &cli.repo);

      let text = match &author_entries[0].message {
        Some(msg) => format!("- {md_author}: {msg} {pr_link}\n"),
        None => format!("- {md_author}: No message could be generated {pr_link}\n"),
      };
      output.push_str(&text);
    }
  }

  let current_dir = env::current_dir()?;
  let directory_path = current_dir.join("output");
  let filepath = directory_path.join(format!("{}.md", get_current_date()));

  fs::create_dir_all(directory_path).unwrap();
  fs::write(&filepath, output).unwrap();

  println!("Successfully created {}", &filepath.display());

  Ok(())
}

#[derive(Parser)]
#[clap(
  author = "LekoArts",
  name = "@lekoarts/thanks-contributors",
  about = "Generate a list of contributors for a commit range",
  long_about = "This little script accesses GitHub's API to get all contributors and their PRs between two distinct points in the history of commits. This is helpful for changelogs where you'd want to list all contributions for that release (so e.g. changes between v1 and v1.1)."
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
  /// Include organization members into the list [default: false]
  #[clap(short, long)]
  include_org_members: Option<bool>,
  /// List of members to exclude from the list. Usage: -e=member1,member2 [default: "renovate-bot", "renovate[bot]"]
  #[clap(
    short,
    long,
    num_args = 1..,
    use_value_delimiter = true,
    value_delimiter = ',',
  )]
  excludes: Option<Vec<String>>,
  #[clap(flatten)]
  verbose: Verbosity,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
  }
}
