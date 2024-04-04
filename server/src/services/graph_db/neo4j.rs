//! Connections to an instance of Neo4j

use futures::executor;

use super::{GraphDbConnection, GraphDbDriver};
use crate::local::*;
use wrangler_common::{configuration::apps::neo4j::*, prelude::Result as AWResult};

// use futures::stream::*;
use neo4rs::*;
// use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
// use uuid::Uuid;

pub struct Neo4jConnection {
  // /// Tokio::Runtime - an async loop for temporarily making the connection synchronous.
  // rt: tokio::runtime::Runtime,
  graph: Arc<Graph>,
}

impl Neo4jConnection {
  pub async fn init(driver: &Neo4jConfig) -> AWResult<Self> {
    let uri = driver.get_uri();
    let user = driver.get_username();
    let pass = driver.get_password().value().to_string();

    // let rt = tokio::runtime::Builder::new_current_thread()
    //   .enable_all()
    //   .build()?;

    // // Doing this manually because adding the neo4j library to common breaks
    // let graph = rt.block_on(Graph::new(uri, user, pass)).map_err(|err| {
    //   let result: AllWhat<WranglerErrorKind> = WranglerErrorKind::GraphDbError.into();
    //   result.set_context(&format!("From <std::io::Error>:\n{:#?}", err))
    // })?;
    println! {"Connecting to graph with {:?}, {:?}, {:#?}", &uri, user, pass};
    let graph = Graph::new(&uri, user, pass).await.unwrap();

    Ok(Neo4jConnection {
      graph: Arc::new(graph),
    })
  }
}

impl Neo4jConnection {}

impl GraphDbConnection for Neo4jConnection {
  async fn create(
    &self,
    _node: Box<dyn wrangler_common::grapht::prelude::GraphtNode>,
  ) -> AWResult<()> {
    todo!()
  }

  async fn relate(
    &self,
    _edge: Box<dyn wrangler_common::grapht::prelude::GraphtEdge>,
  ) -> AWResult<()> {
    todo!()
  }

  // A simple query to return a list of matching nodes
  async fn find(&self, search: String) -> AWResult<()> {
    let graph = self.graph.clone();
    let search = search.clone();
    println!(
      "In the GraphDbConnection.find function. Running query {:#?}",
      search
    );

    println!("Inside the async");
    let q = query(&search);
    let mut result = graph.execute(q).await.unwrap();
    println!("Received a result");
    while let Ok(Some(row)) = result.next().await {
      let node: Node = row.get("p").unwrap();
      println!("Got a node: {:?}", node);
    }

    println!("Post spawn");
    Ok(())
  }

  async fn ping(&self) -> AWResult<()> {
    println!("In the GraphDbConnection.ping function");
    self.find("MATCH (p:Ping) RETURN p".to_string()).await
  }
}

impl GraphDbDriver for Neo4jConfig {
  type Connection = Neo4jConnection;

  fn init(&self) -> AWResult<()> {
    todo!("GraphDbDriver::init for Neo4jConfig")
  }

  fn get_connection(&self, _db_name: &str) -> AWResult<Neo4jConnection> {
    let conn = executor::block_on(Neo4jConnection::init(self))?;
    Ok(conn)
  }

  fn ping(&self) -> AWResult<()> {
    let conn = self.get_connection("PingTest")?;
    futures::executor::block_on(conn.ping())
  }
}
