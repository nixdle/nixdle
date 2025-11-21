#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("request: {0}")]
  Reqwest(#[from] reqwest::Error),
  #[error("io: {0}")]
  Io(#[from] std::io::Error),
  #[error("json: {0}")]
  Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
