//! Pretend that a user is clicking around the database from scratch
//!
//! This integration test emulates the calls that a GUI might make as a user enters a submission
//! from a clean install

use wrangler_common::{configuration::apps::neo4j::*, model::SubmissionLog};
use wrangler_server::services::graph_db::{Driver, GraphDb};

// fn workspace_init() {
//   println!("Initializing the workspace");
//   let config = WorkspaceConfig::default();
//   println!("{config}");

//   let workspaces = Workspace::init(config).unwrap();
//   println!("{workspace}")
// }

#[tokio::test]
async fn happy_path() {
  // let graph = neo4rs::Graph::new("localhost:7687", "neo4j", "neo_pass")
  //   .await
  //   .unwrap();
  // let graph = graph.clone();
  // tokio::spawn(async move {
  //   println!("Calling the initial graph");
  //   let mut result = graph
  //     .execute(neo4rs::query("MATCH (p:Ping) RETURN p"))
  //     .await
  //     .unwrap();

  //   println!("Finished calling the initial graph");
  //   while let Ok(Some(row)) = result.next().await {
  //     let node: neo4rs::Node = row.get("p").unwrap();
  //     // println!("{:#?}", node);
  //   }
  // });

  // Make the workspace
  // - Connect to the database
  let driver = match Neo4jConfig::new("localhost:7687", "neo4j", "neo_pass") {
    Ok(config) => Driver::Neo4j(config),
    Err(err) => panic!("Failed to make the Neo4jConfig driver:\n{:#?}", err),
  };

  let graph = GraphDb::open(driver, "HappyPathTest").await.unwrap();

  // // Directly test ping
  // let _ping_id = uuid::Uuid::new_v4();

  // // Retrieve all ping (count should be 0 or 1)
  // println!("\nAbout to test the ping");
  // graph.ping().unwrap();

  // Retrieve all the organizations

  // Add a root organization using the DSL

  // Query all the organizations

  // Call import
  // Re-query the organizations

  // Ensure everything finishes before exiting the test

  println!("Calling the final sleep");
  tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
}
