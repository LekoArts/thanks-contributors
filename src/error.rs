use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThxContribError {
  #[error("{reason}")]
  ClapError {
    #[source]
    source: clap::Error,
    reason: String,
  },
  #[error(transparent)]
  Var(#[from] std::env::VarError),
  #[error(transparent)]
  NapiError(#[from] napi::Error),
}

impl From<ThxContribError> for napi::Error {
  fn from(e: ThxContribError) -> Self {
    napi::Error::from_reason(format!("{}", e))
  }
}

impl ThxContribError {
  pub fn cli_error<I: clap::IntoApp>(err: clap::Error) -> Self {
    let mut app = I::into_app();
    let reason = err.format(&mut app);

    Self::ClapError {
      reason: "CLI Error".to_owned(),
      source: reason,
    }
  }
}
