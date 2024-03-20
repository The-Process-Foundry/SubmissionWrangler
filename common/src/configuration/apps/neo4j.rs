//! Data for connecting to a specific tree in a neo4j service

use crate::local::*;

use super::primitives::{password::*, uri::*};

#[derive(Debug, Clone)]
pub struct Neo4jConfig {
  bolt_uri: UriConfig,
  user: &'static str,
  password: Password,
}

impl Neo4jConfig {
  pub fn new(uri: &'static str, user: &'static str, password: &'static str) -> Result<Neo4jConfig> {
    let uri_config = UriConfig::new(uri).constrain(vec![UriConstraint::IsUrl])?;

    Ok(Neo4jConfig {
      bolt_uri: uri_config,
      user,
      password: Password::new(password),
    })
  }

  pub fn get_uri(&self) -> String {
    self.bolt_uri.clone().into()
  }

  pub fn get_username(&self) -> String {
    self.user.into()
  }

  pub fn get_password(&self) -> Password {
    self.password.clone()
  }
}

// Set the default for localhos
impl Default for Neo4jConfig {
  fn default() -> Neo4jConfig {
    Neo4jConfig::new("127.0.0.1:7687", "neo4j", "neo4j").unwrap()
  }
}
