//! A rust implementation of the Wrangler server API spec

use crate::longrunner::{LongRunnerRouter, LongRunnerTask};
use serde::{Deserialize, Serialize};
// Database operations
pub mod data;

/// Asynchronous calls that should be handled by the LongRunner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LongRunnerCall {
  ImportCSV,
  PrintInvoice,
}

impl LongRunnerRouter for LongRunnerCall {
  fn from_str(action: String) -> Result<Self, String> {
    serde_json::from_str(&action)
      .map_err(|err| format!("Failed to deserialize action:\n{}\n{}", action, err))
  }

  fn to_task(&self) -> Result<LongRunnerTask<LongRunnerCall>, String> {
    match self {
      LongRunnerCall::ImportCSV => todo!(),
      LongRunnerCall::PrintInvoice => todo!("PrintInvoice task"),
    }
  }
}

/// Top level routing data for the system
///
/// In essence, each of these is the equivalent of an API endpoint.
pub enum Call {
  Settings,
  Data(data::Query),
  Heartbeat,
}
