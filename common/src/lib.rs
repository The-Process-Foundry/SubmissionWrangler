//! Library to facilitate communication between the client and server portions of the wrangler

// The description of information stored in the database
pub mod model;

// Setup for tools needed by the system and a method of communicating about them
pub mod configuration;

// An aggregate of all the wrangler errors
pub mod errors;

// Functions that are exposed via an endpoint
pub mod calls;

// Opinionated tooling to keep things consistent
pub mod tools;

// Database design
pub mod grapht;

// Import the most used definitions
pub mod prelude {
  pub use super::{calls, configuration, errors, model};

  // Everything should be using the same error kinds
  pub use super::errors::WranglerErrorKind::{self, *};

  // This will be split out into it's own crate
  pub use crate::errors::AllWhat;

  pub type Result<O> = core::result::Result<O, AllWhat<WranglerErrorKind>>;
}

/// Re-exports for sharing internal domain types and configuration
pub(crate) mod local {
  // Everything should be using the same error kinds
  pub use super::errors::WranglerErrorKind::{self, *};

  // This will be split out into it's own crate
  pub use crate::errors::{allwhat::ResultPlus, AllWhat};

  pub type Result<O> = core::result::Result<O, AllWhat<WranglerErrorKind>>;

  // pub use core::fmt::{self, Debug, Display};
  pub use core::fmt::Debug;
}
