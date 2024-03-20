//! Synchronize local values with the graph database
//!
//! This currently does not follow edges for updates and must be handled by the user manually

/// Your basic CRUD management of the database
#[derive(Debug)]
pub enum Query {
  Create,
  Delete,
  Retrieve,
  Update,
  Upsert,
}
