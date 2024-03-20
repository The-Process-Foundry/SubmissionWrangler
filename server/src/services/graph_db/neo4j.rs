//! Connections to an instance of Neo4j
//!
//! Building this as a blocking client for now until I decide how to integrate Tokio

use super::{GraphDbConnection, GraphDbDriver};
use crate::local::*;
use wrangler_common::{configuration::apps::neo4j::*, prelude::Result as AWResult};

// use futures::stream::*;
use neo4rs::*;
// use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
// use uuid::Uuid;

pub struct Neo4jConnection {
  /// Tokio::Runtime - an async loop for temporarily making the connection synchronous.
  rt: tokio::runtime::Runtime,

  graph: Arc<Graph>,
}

impl Neo4jConnection {
  pub fn init(driver: &Neo4jConfig) -> AWResult<Self> {
    let uri = driver.get_uri();
    let user = driver.get_username();
    let pass = driver.get_password().to_string();

    let rt = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()?;

    // Doing this manually because adding the neo4j library to common breaks
    let graph = rt.block_on(Graph::new(uri, user, pass)).map_err(|err| {
      let result: AllWhat<WranglerErrorKind> = WranglerErrorKind::GraphDbError.into();
      result.set_context(&format!("From <std::io::Error>:\n{:#?}", err))
    })?;

    Ok(Neo4jConnection {
      rt,
      graph: Arc::new(graph),
    })
  }
}

impl GraphDbConnection for Neo4jConnection {
  fn create(&self, node: Box<dyn wrangler_common::grapht::prelude::GraphtNode>) -> AWResult<()> {
    todo!()
  }

  fn relate(&self, edge: Box<dyn wrangler_common::grapht::prelude::GraphtEdge>) -> AWResult<()> {
    todo!()
  }

  fn find(&self, query: &str) {
    todo!()
  }
}

impl GraphDbDriver for Neo4jConfig {
  type Connection = Neo4jConnection;

  fn connect(&self) -> AWResult<()> {
    Neo4jConnection::init(self)?;
    Ok(())
  }

  fn get_connection(&self) -> AWResult<Neo4jConnection> {
    todo!("GraphDb Driver - get connection")
  }
}
