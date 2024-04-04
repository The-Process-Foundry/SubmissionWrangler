//! Connections to a graph database

use crate::local::*;

use wrangler_common::{configuration::apps::neo4j::*, grapht::prelude::*};

pub mod neo4j;
mod ping;
mod gql;

use neo4j::Neo4jConnection;

/// A model that can be serialized/deserialized into the graph
pub trait Grapht {}

/// A common interface tha all Graph Databases are expected to implement. It is meant to grab a
/// connection from a backend pool.
pub trait GraphDbConnection {
  async fn create(&self, node: Box<dyn GraphtNode>) -> Result<()>;

  async fn relate(&self, edge: Box<dyn GraphtEdge>) -> Result<()>;

  async fn find(&self, query: String) -> Result<()>;

  /// Check that a connection is up, running, and taking queries
  async fn ping(&self) -> Result<()>;
}

/// How to create a specific connection value based on a config
pub trait GraphDbDriver {
  type Connection: GraphDbConnection;

  /// Initialize a connection pool and verify the driver settings
  fn init(&self) -> Result<()>;

  // Open a connection to a specific graph in the database
  fn get_connection(&self, db_name: &str) -> Result<Self::Connection>;

  /// A simple check to make sure the connection is up and running
  fn ping(&self) -> Result<()>;
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
      Driver::Neo4j(driver) => driver.get_connection(db_name),
    }
  }
}

impl Default for Driver {
  fn default() -> Driver {
    Driver::Neo4j(Neo4jConfig::default())
  }
}

/// A generic interface for interacting with a single graph.
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
  async fn init(&mut self) -> Result<()> {
    // Open the connection
    self.connection = Some(self.driver.connect(&self.db_name)?);

    // Test the connection with a ping
    println!("Running GraphDb::init ping");
    self.ping().await?;
    Ok(())
  }

  /// Creates a frontend for communicating with a graph database
  pub async fn open(driver: Driver, graph_name: &str) -> Result<GraphDb> {
    let mut graph = GraphDb {
      driver,
      connection: None,
      db_name: graph_name.to_string(),
    };

    graph.init().await?;

    Ok(graph)
  }

  /// Send a trivial create/retrieve to the backend database to ensure the connection is alive and
  /// functioning properly
  pub async fn ping(&self) -> Result<()> {
    if let Some(conn) = &self.connection {
      println!("In the GraphDb::ping function");
      conn.ping().await
    } else {
      panic!("Tried to ping before creating the connection")
    }

    // Query the ping singleton.

    // Upsert a ping with a new uuid
    // Query the ping singleton and ensure it has the new uuid
  }

  /// Takes a raw query string and executes it. This is done synchronously and the result is not returned.
  pub async fn exec(&self, query: &str) -> Result<()> {
    println!("Executing query: {}", query);
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
