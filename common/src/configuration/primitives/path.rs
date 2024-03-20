//! Choose file and paths
//!
//! This functionality changes based on whether it is used in a browser sandbox

use crate::local::*;

#[derive(Debug, Clone)]
pub enum PathConstraint {
  Not(Box<PathConstraint>),
  IsFile,
  IsDirectory,
}

impl PathConstraint {
  pub fn test(&self, _value: &'static str) -> Result<()> {
    Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct PathConfig {
  path: &'static str,
  constraints: Vec<PathConstraint>,
}

impl PathConfig {
  pub fn new(path: &'static str) -> PathConfig {
    // let result = Path::try_from(Path);
    PathConfig {
      path,
      constraints: Vec::new(),
    }
  }

  pub fn is_valid(&self) -> bool {
    self
      .constraints
      .iter()
      .find(|constraint: &&PathConstraint| constraint.test(&self.path).is_err())
      .is_none()
  }

  pub fn validate(self) -> Result<Self> {
    // Test the value against the new constraints
    let results: Vec<Result<()>> = self
      .constraints
      .iter()
      .map(|constraint: &PathConstraint| constraint.test(&self.path))
      .collect();

    let _ = AllWhat::flatten(ValidationError, results)
      .set_context(&format!("Path '{}' failed validation", self.path))?;

    Ok(self)
  }

  pub fn constrain(self, constraints: Vec<PathConstraint>) -> Result<Self> {
    PathConfig {
      constraints,
      ..self
    }
    .validate()
  }
}
