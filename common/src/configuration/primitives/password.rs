//! A simple string wrapper which does not automatically get printed to screen

use core::fmt::{Debug, Display};

/// A value that should never be printed out to a log or screen
#[derive(Clone)]
pub struct Password(&'static str);

impl Password {
  pub fn new(password: &'static str) -> Password {
    Password(password)
  }

  pub fn value(&self) -> &'static str {
    self.0
  }
}

impl Display for Password {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "*******")
  }
}

impl Debug for Password {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "*******")
  }
}
