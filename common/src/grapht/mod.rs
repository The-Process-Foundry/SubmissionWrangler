//! Tools for creating an in-memory Graph Db

pub trait GraphtPayload {}

pub trait GraphtNode {}

pub trait GraphtEdge {}

pub mod prelude {
  pub use super::{GraphtEdge, GraphtNode, GraphtPayload};
}
