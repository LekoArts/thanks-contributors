#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use clap::{AppSettings, FromArgMatches, IntoApp, Parser};
use dotenv::dotenv;
use napi::bindgen_prelude::*;
use reqwest::header::CONTENT_TYPE;
use std::env;

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

fn format_cli_error<I: IntoApp>(err: clap::Error) -> clap::Error {
  let mut app = I::into_app();
  err.format(&mut app)
}

#[napi]
async fn run(args: Vec<String>) -> Result<()> {
  dotenv().ok();
  let app = Cli::into_app();
  let matches = app.get_matches_from(args);
  let res = Cli::from_arg_matches(&matches).map_err(format_cli_error::<Cli>);
  let cli = match res {
    Ok(s) => s,
    Err(e) => e.exit(),
  };

  let env_var_name = "GITHUB_ACCESS_TOKEN";
  let gh_token = match env::var(env_var_name) {
    Ok(token) => Ok(token),
    Err(e) => Err(Error::new(
      Status::GenericFailure,
      format!("{} is not set ({})", env_var_name, e),
    )),
  };

  println!(
    "base: {:?} - head: {:?} - owner: {:?} - repo: {:?}",
    cli.base, cli.head, cli.owner, cli.repo
  );

  Ok(())
}
