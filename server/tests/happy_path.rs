//! Pretend that a user is clicking around the database from scratch
//!
//! This integration test emulates the calls that a GUI might make as a user enters a submission
//! from a clean install

use wrangler_common::configuration::apps::neo4j::*;
use wrangler_server::{
  prelude::*,
  services::graph_db::{neo4j::*, Driver},
};

fn workspace_init() {
  println!("Initializing the workspace");
  let config = WorkspaceConfig::default();
  println!("{config}");

  let workspaces = Workspace::init(config).unwrap();
  println!("{workspace}")
}

#[test]
fn happy_path() {
  let config = Neo4jConfig.new("127.0.0.1:7687", "neo4j", "neo4j");

  // Make the workspace
  // workspace_init();

  // Query the organizations

  // Call import
  // Re-query the organizations
}
