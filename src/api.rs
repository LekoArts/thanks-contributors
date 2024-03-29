use napi::bindgen_prelude::Result;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::Deserialize;

use crate::error::reqwest_error;

pub async fn compare_commits(
  owner: &str,
  repo: &str,
  base: String,
  head: String,
  gh_token: &str,
) -> Result<Vec<Commit>> {
  let client = reqwest::Client::new();
  let response = client
    .get(format!(
      "https://api.github.com/repos/{owner}/{repo}/compare/{base}...{head}"
    ))
    .header(USER_AGENT, "thanks-contributors")
    .header(AUTHORIZATION, format!("token {gh_token}"))
    .send()
    .await
    .map_err(reqwest_error)?
    .json::<CompareCommitsResponse>()
    .await
    .map_err(reqwest_error)?;

  Ok(response.commits)
}

pub async fn list_members(owner: &str, gh_token: &str) -> Result<Vec<String>> {
  let client = reqwest::Client::new();
  let response = client
    .get(format!(
      "https://api.github.com/orgs/{owner}/members?per_page=100"
    ))
    .header(USER_AGENT, "thanks-contributors")
    .header(AUTHORIZATION, format!("token {gh_token}"))
    .send()
    .await
    .map_err(reqwest_error)?
    .json::<Vec<Member>>()
    .await
    .map_err(reqwest_error)?;

  let list_of_logins = response.into_iter().map(|m| m.login).collect();

  Ok(list_of_logins)
}

#[derive(Debug, Deserialize)]
pub struct CompareCommitsResponse {
  pub commits: Vec<Commit>,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
  pub url: String,
  pub commit: CommitMeta,
  pub author: Option<CommitAuthor>,
}

#[derive(Debug, Deserialize)]
pub struct CommitMeta {
  pub url: String,
  pub message: String,
  pub author: CommitMetaAuthor,
}

#[derive(Debug, Deserialize)]
pub struct CommitMetaAuthor {
  pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CommitAuthor {
  pub login: String,
  pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Member {
  pub login: String,
}
