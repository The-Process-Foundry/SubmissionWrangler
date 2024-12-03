//! Connections to an instance of Neo4j

use futures::executor;

use super::{GraphDbConnection, GraphDbDriver};
use wrangler_common::{configuration::apps::neo4j::*, prelude::Result as AWResult};

// use futures::stream::*;
use neo4rs::*;
// use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
// use uuid::Uuid;

#[derive(Clone)]
pub struct Neo4jConnection {
  // /// Tokio::Runtime - an async loop for temporarily making the connection synchronous.
  // rt: tokio::runtime::Runtime,
  graph: Arc<Graph>,
}

impl std::fmt::Debug for Neo4jConnection {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("Graph")
      .field(&"There is data somewhere")
      .finish()
  }
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
  async fn _create(
    &self,
    _node: Box<dyn wrangler_common::grapht::prelude::GraphtNode>,
  ) -> AWResult<()> {
    todo!()
  }

  async fn _relate(
    &self,
    _edge: Box<dyn wrangler_common::grapht::prelude::GraphtEdge>,
  ) -> AWResult<()> {
    todo!()
  }

  async fn exec(&self, query_str: String) -> AWResult<()> {
    let graph = self.graph.clone();

    println!("Exec query: {:#?}", query_str);
    let q = query(&query_str);
    let mut res = graph.execute(q).await.unwrap();
    println!("Exec returned: {:?}", &res.next().await);
    Ok(())
  }

  // A simple query to return a list of matching nodes
  async fn find(&self, search: String) -> AWResult<Vec<Row>> {
    let graph = self.graph.clone();
    let search = search.clone();
    println!(
      "In the GraphDbConnection.find function. Running query {:#?}",
      search
    );

    let q = query(&search);
    let mut result = graph.execute(q).await.unwrap();
    let mut rows = vec![];
    while let Ok(Some(row)) = result.next().await {
      let node: Node = row.get("p").unwrap();
      println!("Got a node: {:?}", node);
      rows.append(&mut vec![row]);
    }
    Ok(rows)
  }

  async fn ping(&self) -> AWResult<()> {
    println!("In the GraphDbConnection.ping function");

    // Make a new ping ID to verify read/write works
    let ping_id = uuid::Uuid::new_v4();
    let query = format!("MATCH (p:PING WHERE p.guid = '{}') RETURN p", ping_id);

    // Check there are no pings with this id
    let initial = self.find(query.clone()).await?;
    match initial.len() {
      0 => (),
      _ => panic!("This should not exist yet"),
    };

    // Insert the ping and set it to have the new guid
    let upsert = format!(
      "MERGE (p:PING) ON CREATE SET p.guid = '{}' ON MATCH SET p.guid = '{}'",
      ping_id, ping_id
    );
    let _ = self.exec(upsert).await?;

    // Verify there is now a ping with that guid
    let initial = self.find(query).await?;
    match initial.len() {
      1 => (),
      x => panic!(
        "There should be exactly one ping with guid {}, but found {}",
        ping_id, x
      ),
    };

    Ok(())
  }
}

impl GraphDbDriver for Neo4jConfig {
  type Connection = Neo4jConnection;

  fn _init(&self) -> AWResult<()> {
    todo!("GraphDbDriver::init for Neo4jConfig")
  }

  fn get_connection(&self, _db_name: &str) -> AWResult<Neo4jConnection> {
    let conn = executor::block_on(Neo4jConnection::init(self))?;
    Ok(conn)
  }

  fn _ping(&self) -> AWResult<()> {
    let conn = self.get_connection("PingTest")?;
    futures::executor::block_on(conn.ping())
  }
}
