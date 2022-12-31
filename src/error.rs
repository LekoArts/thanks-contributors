use std::env::VarError;

use clap::CommandFactory;
use napi::Error as NapiError;

pub fn format_cli_error<I: CommandFactory>(err: clap::Error) -> NapiError {
  let mut app = I::command();
  let reason = err.format(&mut app);

  NapiError::from_reason(format!("{reason}"))
}

pub fn env_var_error(err: VarError) -> NapiError {
  match err {
    VarError::NotPresent => {
      NapiError::from_reason("Environment variable 'GITHUB_ACCESS_TOKEN' not present".to_owned())
    }
    VarError::NotUnicode(_) => {
      NapiError::from_reason("Environment variable 'GITHUB_ACCESS_TOKEN' not unicode".to_owned())
    }
  }
}

pub fn reqwest_error(err: reqwest::Error) -> NapiError {
  if err.is_timeout() {
    NapiError::from_reason(format!("Request timed out: {err}"))
  } else if err.is_status() {
    NapiError::from_reason(format!(
      "Unexpected status code {code}: {err}",
      code = err.status().unwrap_or_default().as_u16(),
      err = err
    ))
  } else if err.is_decode() {
    NapiError::from_reason(format!("Failed to parse response body: {err}"))
  } else if err.is_body() {
    NapiError::from_reason(format!("Failed to serialize request body: {err}"))
  } else {
    NapiError::from_reason(format!(
      "An unknown error occurred while sending the request: {err}"
    ))
  }
}
