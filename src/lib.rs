#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use crate::error::ThxContribError;
use clap::{AppSettings, FromArgMatches, IntoApp, Parser};
use dotenv::dotenv;
use napi::bindgen_prelude::*;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use std::env;

mod error;

#[derive(Parser)]
#[clap(
  author = "LekoArts",
  name = "@lekoarts/thanks-contributors",
  about = "This little script accesses GitHub's API to get all contributors and their PRs between two distinct points in the history of commits. This is helpful for changelogs where you'd want to list all contributions for that release (so e.g. changes between v1 and v1.1)."
)]
#[clap(global_setting(AppSettings::NoBinaryName))]
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
}

const ENV_VAR_NAME: &str = "GITHUB_ACCESS_TOKEN";

#[napi]
async fn run(args: Vec<String>) -> Result<()> {
  dotenv().ok();
  let app = Cli::into_app();
  let matches = app.get_matches_from(args);
  let cli = Cli::from_arg_matches(&matches).map_err(|e| ThxContribError::cli_error::<Cli>(e))?;

  let gh_token = env::var(ENV_VAR_NAME).map_err(|e| ThxContribError::from(e))?;

  let client = reqwest::Client::new();

  let response = client
    .get(
      "https://api.github.com/repos/gatsbyjs/gatsby/compare/master...memoize-cache-date-formatting",
    )
    .header(USER_AGENT, "thanks-contributors")
    .header(AUTHORIZATION, format!("token {}", gh_token))
    .send()
    .await
    .map_err(|e| ThxContribError::reqwest_error(e))?
    .json::<serde_json::Value>()
    .await
    .map_err(|e| ThxContribError::reqwest_error(e))?;

  dbg!(response);

  Ok(())
}
