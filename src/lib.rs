#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use dotenv::dotenv;
use napi::bindgen_prelude::*;
use reqwest::header::CONTENT_TYPE;
use std::env;

#[napi]
async fn run() -> Result<()> {
  dotenv().ok();

  let env_var_name = "GITHUB_ACCESS_TOKEN";
  let gh_token = match env::var(env_var_name) {
    Ok(token) => Ok(token),
    Err(e) => Err(Error::new(
      Status::GenericFailure,
      format!("{} is not set ({})", env_var_name, e),
    )),
  };

  println!("{:?}", gh_token);

  let client = reqwest::Client::new();

  let response = client
    .get("https://hp-graphql.netlify.app/.netlify/functions/graphql")
    .header(CONTENT_TYPE, "application/graphql")
    .send()
    .await?
    .json()
    .await?;

  Ok(())
}
