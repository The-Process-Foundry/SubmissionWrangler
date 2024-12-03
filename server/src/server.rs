//! A singleton object to control IO to and from the client

use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tracing::info;

use crate::{longrunner::LongRunner, workspace::Workspace};
use wrangler_common::{calls::LongRunnerCall, longrunner::LongRunnerTask};

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
/// includes managing all related services such as the LongRunner and the DB pool.
pub struct Server {
  pub long_runner: LongRunner<LongRunnerCall>,
  pub workspace: Workspace,
}

fn executor(task: Arc<RwLock<LongRunnerTask<LongRunnerCall>>>, ctx: Workspace) {
  let inner = task.read().unwrap();
  match inner.task_route {
    LongRunnerCall::ImportCSV => crate::runners::import_csv::runner(inner.state.clone(), ctx),
    LongRunnerCall::PrintInvoice => todo!(),
  }
}

impl Server {
  pub fn create() -> Server {
    let executor = Arc::new(executor);
    let workspace = Workspace::default();
    let long_runner = LongRunner::<LongRunnerCall>::new(executor, workspace.clone());
    Server {
      long_runner,
      workspace,
    }
  }

  fn ping(&self) -> String {
    "Pong".to_string()
  }

  /// Handler for incoming messages. This should return a response as quickly as possible. Any
  /// long-running processes should be spawned off and an identifier for the process should be
  /// returned instead of the final value.
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
