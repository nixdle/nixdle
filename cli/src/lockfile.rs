use std::{fs, path::Path};

use crate::crypto::verify_hmac;
use color_eyre::Result;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Lockfile {
  pub date: String,
  pub success: bool,
  pub attempts: usize,
  pub attempted: Vec<String>,
  pub version: String,
}

impl Lockfile {
  pub fn open(key: &str) -> Result<Self> {
    let path = Path::new(crate::LOCKFILE_PATH);
    let signature_path = Path::new(crate::LOCKFILE_SIGNATURE_PATH);

    if path.exists() {
      let contents = fs::read_to_string(path)?;
      let signature = fs::read(signature_path)?;

      if verify_hmac(key, &contents, &signature) {
        return Ok(serde_json::from_str(&contents)?);
      }
    }

    Ok(Lockfile::default())
  }

  pub fn save(&self, key: &str) -> Result<()> {
    let path = Path::new(crate::LOCKFILE_PATH);
    let signature_path = Path::new(crate::LOCKFILE_SIGNATURE_PATH);

    let contents = serde_json::to_string(self)?;
    let signature = crate::crypto::generate_hmac(key, &contents);

    fs::write(path, contents)?;
    fs::write(signature_path, signature)?;

    Ok(())
  }

  fn default() -> Self {
    Lockfile {
      date: String::new(),
      success: false,
      attempts: 0,
      attempted: Vec::new(),
      version: String::new(),
    }
  }
}
