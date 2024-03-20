//! A wrapper for identifying the location of an endpoint (file, url, socket, etc.)

use http::uri::Uri;

use crate::local::*;

#[derive(Debug, Clone)]
pub enum UriConstraint {
  Not(Box<UriConstraint>),
  IsUrl,
}

impl UriConstraint {
  pub fn test(&self, _value: &Uri) -> Result<()> {
    Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct UriConfig {
  uri: Uri,
  constraints: Vec<UriConstraint>,
}

impl UriConfig {
  pub fn new(uri: &str) -> UriConfig {
    let result = Uri::try_from(uri);
    match result {
      Ok(value) => UriConfig {
        uri: value,
        constraints: Vec::new(),
      },
      Err(err) => panic!("Invalid uri given: {:?}", err),
    }
  }

  pub fn is_valid(&self) -> bool {
    self
      .constraints
      .iter()
      .find(|constraint: &&UriConstraint| constraint.test(&self.uri).is_err())
      .is_none()
  }

  pub fn validate(self) -> Result<Self> {
    // Test the value against the new constraints
    let results: Vec<Result<()>> = self
      .constraints
      .iter()
      .map(|constraint: &UriConstraint| constraint.test(&self.uri))
      .collect();

    let _ = AllWhat::flatten(ValidationError, results)
      .set_context(&format!("Uri '{}' failed validation", self.uri))?;

    Ok(self)
  }

  pub fn constrain(self, constraints: Vec<UriConstraint>) -> Result<Self> {
    UriConfig {
      constraints,
      ..self
    }
    .validate()
  }
}

impl From<UriConfig> for String {
  fn from(value: UriConfig) -> Self {
    value.uri.to_string()
  }
}
