use napi::Error as NapiError;
use reqwest::Error as ReqwestError;
use std::env::VarError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThxContribError {
  #[error("failed to parse response body: {0}")]
  Deserialize(#[source] ReqwestError),
  #[error("token has incorrect permissions")]
  InvalidPermissions,
  #[error("failed to serialize request body: {0}")]
  Serialize(#[source] ReqwestError),
  #[error("unexpected status code {code}")]
  Status { code: u16, source: ReqwestError },
  #[error("request timed out: {0}")]
  Timeout(#[source] ReqwestError),
  #[error("an unknown error occurred while sending the request: {0}")]
  Unknown(#[source] ReqwestError),
  #[error("{reason}")]
  ClapError {
    #[source]
    source: clap::Error,
    reason: String,
  },
  #[error(transparent)]
  Var(#[from] VarError),
  #[error(transparent)]
  NapiError(#[from] NapiError),
}

impl From<ThxContribError> for NapiError {
  fn from(e: ThxContribError) -> Self {
    napi::Error::from_reason(format!("{}", e))
  }
}

impl ThxContribError {
  pub fn cli_error<I: clap::CommandFactory>(err: clap::Error) -> Self {
    let mut app = I::command();
    let reason = err.format(&mut app);

    Self::ClapError {
      reason: "CLI Error".to_owned(),
      source: reason,
    }
  }

  pub fn reqwest_error(err: ReqwestError) -> Self {
    if err.is_timeout() {
      ThxContribError::Timeout(err)
    } else if err.is_status() {
      ThxContribError::Status {
        code: err.status().unwrap_or_default().as_u16(),
        source: err,
      }
    } else if err.is_decode() {
      ThxContribError::Deserialize(err)
    } else if err.is_body() {
      ThxContribError::Serialize(err)
    } else {
      ThxContribError::Unknown(err)
    }
  }
}
