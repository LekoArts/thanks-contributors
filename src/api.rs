use crate::error::ThxContribError;
use napi::bindgen_prelude::Result;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::Deserialize;

pub async fn compare_commits(
  owner: &str,
  repo: &str,
  base: String,
  head: String,
  gh_token: &str,
) -> Result<CompareCommitsResponse> {
  let client = reqwest::Client::new();
  let response = client
    .get(format!(
      "https://api.github.com/repos/{}/{}/compare/{}...{}",
      owner, repo, base, head
    ))
    .header(USER_AGENT, "thanks-contributors")
    .header(AUTHORIZATION, format!("token {}", gh_token))
    .send()
    .await
    .map_err(|e| ThxContribError::reqwest_error(e))?
    .json::<CompareCommitsResponse>()
    .await
    .map_err(|e| ThxContribError::reqwest_error(e))?;

  Ok(response)
}

pub async fn list_members(owner: &str, gh_token: &str) -> Result<Vec<String>> {
  let client = reqwest::Client::new();
  let response = client
    .get(format!(
      "https://api.github.com/orgs/{}/members?per_page=100",
      owner
    ))
    .header(USER_AGENT, "thanks-contributors")
    .header(AUTHORIZATION, format!("token {}", gh_token))
    .send()
    .await
    .map_err(|e| ThxContribError::reqwest_error(e))?
    .json::<Vec<Member>>()
    .await
    .map_err(|e| ThxContribError::reqwest_error(e))?;

  let list_of_logins: Vec<String> = response.into_iter().map(|m| m.login).collect();

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
