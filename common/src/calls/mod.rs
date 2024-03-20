//! A rust implementation of the Wrangler server API spec

// Database operations
pub mod data;

/// Top level routing data for the system
///
/// In essence, each of these is the equivalent of an API endpoint.
pub enum Call {
  Settings,
  Data(data::Query),
  Heartbeat,
}
