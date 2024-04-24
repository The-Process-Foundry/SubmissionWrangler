//! A singleton object to control IO to and from the client

use tracing::info;

#[derive(Clone, Debug)]
pub enum Request {
  /// Stops the server from receiving any further calls
  Halt,
  /// Heartbeat call to ensure all is up and running
  Ping,
}

#[derive(Clone, Debug)]
pub enum Response {
  /// The response from a heartbeat call
  Pong,
}

/// The server acts as a singleton context for handling requests and returning responses. It
/// includes managing all related services
pub struct Server {}

impl Server {
  pub fn create() -> Server {
    Server {}
  }

  // Handler for incoming messages. This should return a response as quickly as possible. Any
  // long-running processes should be spawned off and an identifier for the process should be
  // returned instead of the final value.
  pub async fn handle(&self, request: String) -> String {
    info!("In the handler with request {}", request);

    // Deserialize the request
    // Forward the request to the proper service
    match &request[..] {
      "Ping" => "Pong".to_string(),
      "Halt" => "Stopping".to_string(),
      _ => todo!("This should not be necessary once the request is deserialized instead of a direct string match")
    }
  }
}
