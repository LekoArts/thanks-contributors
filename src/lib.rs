#[macro_use]
extern crate napi_derive;

use crate::api::{compare_commits, list_members};
use crate::error::{env_var_error, format_cli_error};
use crate::utils::{create_entries, create_output, get_current_date, group_by_author};
use clap::{CommandFactory, FromArgMatches, Parser};
use clap_verbosity_flag::Verbosity;
use dotenv::dotenv;
use log::{debug, info};
use napi::bindgen_prelude::{Error as NapiError, Result};
use std::env;
use std::fs;

pub mod api;
pub mod error;
pub mod utils;

#[allow(dead_code)]
#[napi]
async fn run(args: Vec<String>) -> Result<()> {
  // Support .env files
  dotenv().ok();

  // Arguments are coming from bin.js
  let matches = Cli::command().get_matches_from(args);
  let cli = Cli::from_arg_matches(&matches).map_err(format_cli_error::<Cli>)?;

  env_logger::Builder::new()
    .filter_level(cli.verbose.log_level_filter())
    .init();

  // By default, don't include org members
  let should_include_org_members = cli.include_org_members.unwrap_or(false);
  // By default, exclude renovate bot
  let parsed_excludes = match cli.excludes {
    Some(e) => e,
    None => vec!["renovate[bot]".to_string(), "renovate-bot".to_string()],
  };

  debug!("Parsed Excludes: {:#?}", parsed_excludes);

  let gh_token = env::var("GITHUB_ACCESS_TOKEN").map_err(env_var_error)?;

  let commits = compare_commits(&cli.owner, &cli.repo, cli.base, cli.head, &gh_token).await?;
  let org_members = list_members(&cli.owner, &gh_token).await?;

  debug!("Commits: {:#?}", commits);
  debug!("Org members: {:#?}", org_members);

  if commits.is_empty() {
    return Err(NapiError::from_reason(
      "Couldn't find any relevant commits. Are you sure you used the correct head & base?"
        .to_owned(),
    ));
  }

  info!("Fetched {} commits", commits.len());

  let entries = create_entries(
    commits,
    should_include_org_members,
    parsed_excludes,
    org_members,
  );

  info!("Process {} filtered commits", entries.len());

  let groups = group_by_author(entries);
  let output = create_output(groups, &cli.owner, &cli.repo);

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
