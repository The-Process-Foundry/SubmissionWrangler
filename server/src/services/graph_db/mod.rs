//! Connections to a graph database

use crate::local::*;

use wrangler_common::{configuration::apps::neo4j::*, grapht::prelude::*};

pub mod neo4j;
use neo4j::Neo4jConnection;

/// A common interface tha all Graph Databases are expected to implement. It is meant to grab a
/// connection from a backend pool.
pub trait GraphDbConnection {
  fn create(&self, node: Box<dyn GraphtNode>) -> Result<()>;

  fn relate(&self, edge: Box<dyn GraphtEdge>) -> Result<()>;

  fn find(&self, query: &str);
}

/// How to create a specific connection value based on a config
pub trait GraphDbDriver {
  type Connection: GraphDbConnection;

  /// Initialize a connection pool and verify the driver settings
  fn connect(&self, db_name: &str) -> Result<()>;

  fn get_connection(&self) -> Result<Self::Connection>;
}

/// An enumeration of all the implemented graph database drivers
#[derive(Debug, Clone)]
pub enum Driver {
  Neo4j(Neo4jConfig),
  // TODO: This will eventually be an in memory cache of the database.
  // Grapht(GraphtConfig),
}

impl Driver {
  /// Initialize a connection pool for the given driver, targeting the specific database name
  fn connect(&self, db_name: &str) -> Result<Neo4jConnection> {
    match self {
      Driver::Neo4j(driver) => driver.connect(db_name),
    }
  }
}

impl Default for Driver {
  fn default() -> Driver {
    Driver::Neo4j(Neo4jConfig::default())
  }
}

/// A generic interface for interacting with a single graph.
#[derive(Clone)]
pub struct GraphDb {
  /// Configuration for the database
  driver: Driver,

  /// An connection pool for communicating with the defined driver.
  ///
  /// This should be a generic connection, but in the interest of getting the pathway working it is
  /// going to assume a Neo4j database.
  connection: Option<Neo4jConnection>,

  /// The name of the graph in the server, the equivalent of a single database in a relational DB.
  db_name: String,
}

impl GraphDb {
  /// Create a connection pool if it doesn't already exist
  pub fn init(&mut self) -> Result<()> {
    self.connection = Some(self.driver.connect(&self.db_name)?);
    Ok(())
  }

  pub fn query(&self) -> Result<()> {
    Ok(())
  }
}

/// Use a the bolt protocol on a local instance of Neo4j by default
impl Default for GraphDb {
  fn default() -> GraphDb {
    GraphDb {
      driver: Driver::default(),
      connection: None,
      db_name: "DefaultGraph".to_string(),
    }
  }
}
