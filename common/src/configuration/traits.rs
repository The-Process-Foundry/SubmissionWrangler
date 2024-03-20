//! Some common trait definitions to commonize the implementations

use crate::local::*;

pub trait Configuration {
  type Constraint;

  fn is_valid(&self) -> bool {
    self
      .constraints
      .iter()
      .find(|constraint: &&Self::Constraint| constraint.test(&self.uri).is_err())
      .is_none()
  }

  fn validate(&self, constraints: &Vec<Self::Constraint>) -> Result<Self> {
    // Test the value against the new constraints
    let results: Vec<Result<()>> = constraints
      .iter()
      .map(|constraint: &Self::Constraint| constraint.test(&self.uri))
      .collect();

    let _ = AllWhat::flatten(
      ValidationError(format!("Uri '{}' failed validation", self.uri)),
      results,
    )?;

    Ok(UriConfig {
      constraints,
      ..self
    })
  }
}
