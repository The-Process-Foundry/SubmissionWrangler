//! Basic work in progress for connecting to the database

use futures::stream::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use uuid::Uuid;

use neo4rs::*;

pub mod model {
  pub struct Organization {
    id: String,
    pretty_id: String,
    name: String,
  }
}

#[derive(Clone, Debug)]
pub struct Neo4jConfig {
  pub uri: String,
  pub username: String,
  pub password: String,
}

#[derive(Clone)]
pub struct Neo4jConnection {
  // /// Tokio::Runtime - an async loop for temporarily making the connection synchronous.
  // rt: tokio::runtime::Runtime,
  graph: Arc<Graph>,
}

impl Neo4jConnection {
  pub async fn connect(config: Neo4jConfig) -> core::result::Result<Neo4jConnection, String> {
    println!("Connecting to db with config: {:#?}", config);
    let graph = Graph::new(&config.uri, config.username, config.password)
      .await
      .unwrap();

    Ok(Neo4jConnection {
      graph: Arc::new(graph),
    })
  }

  /// Execute a list of queries against the graph
  async fn exec(&self, queries: Vec<Query>) -> core::result::Result<(), String> {
    let mut txn = self.graph.clone().start_txn().await.unwrap();
    match txn.run_queries(queries).await {
      Err(err) => match &err {
        Error::UnexpectedMessage(msg) => {
          println!("{:#?}", msg);
          panic!("Unexpected Message in query")
        }
        _ => panic!("Unexpected syntax error in Query: {:?}", err),
      },

      txn => (),
    }
    match txn.commit().await {
      Err(err) => {
        println!("{:#?}", err);
        panic!("Failed to insert everything")
      }
      txn => Ok(()),
    } //or txn.rollback().await.unwrap()
  }

  // A simple read/write to ensure that the connection is running
  pub async fn ping(&self) -> core::result::Result<(), String> {
    let db = self.clone();
    tokio::spawn(async move {
      println!("Running a ping");
      let ping_guid = Uuid::new_v4();
      let ping_query = format!(
        r#" MERGE (p:Ping {{guid: '{}' }})
            RETURN p
        "#,
        ping_guid
      );

      let ping = db.exec(vec![query(&ping_query)]).await.unwrap();
      println!("Ping returned: {:#?}", ping);
    });
    Ok(())
  }

  pub async fn create(&self) -> core::result::Result<(), String> {
    unimplemented!("'create' still needs to be implemented")
  }

  pub async fn update(&self) -> core::result::Result<(), String> {
    unimplemented!("'update' still needs to be implemented")
  }

  pub async fn upsert(&self) -> core::result::Result<(), String> {
    unimplemented!("'upsert' still needs to be implemented")
  }

  pub async fn read(&self) -> core::result::Result<Vec<model::Organization>, String> {
    unimplemented!("'read' still needs to be implemented")
  }

  pub async fn delete(&self) -> core::result::Result<Vec<model::Organization>, String> {
    unimplemented!("'delete' still needs to be implemented")
  }
}
