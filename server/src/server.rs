//! A singleton object to control IO to and from the client

use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Request {
  /// Stops the server from receiving any further calls
  Halt,
  /// Heartbeat call to ensure all is up and running
  Ping,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Response {
  /// The response from a heartbeat call
  Pong,

  Error(String),
}

/// The server acts as a singleton context for handling requests and returning responses. It
/// includes managing all related services
pub struct Server {}

impl Server {
  pub fn create() -> Server {
    Server {}
  }

  fn ping(&self) -> String {
    "Pong".to_string()
  }

  // Handler for incoming messages. This should return a response as quickly as possible. Any
  // long-running processes should be spawned off and an identifier for the process should be
  // returned instead of the final value.
  pub async fn handle(&self, request: String) -> String {
    info!("In the handler with request '{}'", request);

    // Deserialize the request
    match serde_json::from_str(&request) {
      Ok(req) => match req {
        Request::Ping => self.ping(),
        Request::Halt => "Stopping".to_string(),
      },
      Err(err) => format!("Request Deserialization Error: {}", err),
    }
  }
}
