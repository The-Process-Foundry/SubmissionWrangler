//! A port of the internal database used by InvoicerUI. This is used to build an explicit graph from
//! the FHL submission log.

mod model {
  #[derive(Clone, Debug)]
  pub struct Organization {
    pub guid: uuid::Uuid,
    pub pretty_id: String,
    pub name: String,
    pub children: String,
    pub raw: String,
  }
}

mod grapht {
  use neo4rs::*;
  use std::sync::Arc;

  use super::model;

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
    pub async fn connect(config: Neo4jConfig) -> Neo4jConnection {
      let graph = Graph::new(&config.uri, config.username, config.password)
        .await
        .unwrap();

      Neo4jConnection {
        graph: Arc::new(graph),
      }
    }

    pub async fn insert(&self, orgs: Vec<String>) -> Result<(), String> {
      // let orgs = orgs[0..50].to_vec();
      println!("Neo4j insert Organization: {:#?}", orgs);

      let mut txn = self.graph.clone().start_txn().await.unwrap();
      txn.run_queries(orgs).await.unwrap();

      txn.commit().await.unwrap(); //or txn.rollback().await.unwrap();
      Ok(())
    }

    pub async fn query(&self) -> Result<(), String> {
      let graph = self.graph.clone();
      tokio::spawn(async move {
        println!("Running a query");
        let mut result = graph
          .execute(query("MATCH (p:Organization) RETURN p")) // .param("name", "Mark"))
          .await
          .unwrap();

        while let Ok(Some(row)) = result.next().await {
          let node: Node = row.get("p").unwrap();
          let name: String = node.get("name").unwrap();
          println!("Query Result: {}", name);
        }
      });
      Ok(())
    }
  }
}

use grapht::*;
use model::*;

use std::collections::HashMap;

fn load_orgs(file_name: &str) -> HashMap<i32, Organization> {
  // Create a CSV parser that reads data from stdin.
  let mut rdr = csv::ReaderBuilder::new()
    .delimiter('\t' as u8)
    .has_headers(true)
    .from_path(file_name)
    .unwrap();

  // Loop over each record.
  rdr.records().fold(HashMap::new(), |mut acc, result| {
    // An error may occur, so abort the program in an unfriendly way.
    // We will make this more friendly later!
    let record = result.expect("a CSV record");
    // Print a debug version of the record.
    // println!("{:?}) {:?}", i, record);
    let org = Organization {
      guid: uuid::Uuid::new_v4(),
      pretty_id: record.get(1).unwrap().to_string(),
      name: record.get(2).unwrap().to_string(),
      children: record.get(4).unwrap().to_string(),
      raw: format!("{:#?}", record),
    };
    let old_guid = record
      .get(0)
      .expect("The old guid was not in field #0")
      .parse()
      .expect(&format!(
        "Could not convert record {:?} into an i32",
        org.pretty_id
      ));
    acc.insert(old_guid, org);
    acc
  })
}

#[tokio::main]
async fn main() {
  println!("Starting insert");

  let config = Neo4jConfig {
    uri: "127.0.0.1:7687".to_string(),
    username: "neo4j".to_string(),
    password: "neo_pass".to_string(),
  };

  let conn = grapht::Neo4jConnection::connect(config).await;

  // Empty
  conn.query().await;

  // Load the orgs
  let orgs = load_orgs("data/organizations.tsv");

  // Read the orgs and insert each one
  let inserts: Vec<String> = orgs
    .into_iter()
    .map(|(old_id, org)| {
      format!(
        "CREATE (o:Organization {{
        guid: '{:?}',
        pretty_id: '{}',
        name: '{:?}'
      }})",
        org.guid,
        org.pretty_id,
        // FIXME: This escape doesn't work. Need to use a named param in the insert
        org.name.replace("'", "\\'")
      )
    })
    .collect();

  conn.insert(inserts).await;

  // Contains the orgs
  conn.query().await;

  println!("\n---> Finished Running.\n\n");
}
