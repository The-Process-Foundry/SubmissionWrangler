//! A library that that processes user requests for the Submission Wrangler

// Expose the endpoints
// pub mod api;

// pub mod endpoint

// configuration for pointing to various items
pub mod services;

// Configuration items
pub mod workspace;

pub mod local {
  pub use core::fmt;

  pub use wrangler_common::prelude::*;
}

/// Expose a subset of the system to users of the library
pub mod prelude {
  pub use super::workspace::*;
}
