//! A street address

use super::local::*;

#[derive(Clone, Debug)]
pub struct Address {
  pub guid: Uuid,
  pub street1: String,
  pub street2: Option<String>,
  pub street3: Option<String>,
  pub city: String,
  pub state: String,
  pub zip: String,
}
