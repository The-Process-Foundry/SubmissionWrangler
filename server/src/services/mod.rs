//! Various connections and values that are pooled between all the server's activities

pub mod graph_db;

pub mod docker;

pub enum Service {
  GraphDb(graph_db::GraphDb),
}
