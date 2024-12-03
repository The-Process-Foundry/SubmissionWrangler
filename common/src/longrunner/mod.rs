//! Data transfer objects used by the LongRunner. These are the transport layer between the client
//! and server.

use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// The status of the requested process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
  Queued,
  Running,
  Paused,
  Cancelling,
  Cancelled,
  Finished,
  Failed,
}

/// A standard report back about a given process. The events are messages from the process that have
/// occurred since the last time the longrunner was queried.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskState {
  pub task_id: Uuid,
  pub task_status: TaskStatus,
  pub events: Vec<String>,
}

impl TaskState {
  pub fn incr_state(&self) -> TaskState {
    let length = self.events.len();
    match length < 5 {
      true => {
        let mut new_msgs: Vec<String> = vec![];
        for i in length..length * 2 + 1 {
          new_msgs.extend([format!("New Message {}", i)]);
        }

        TaskState {
          task_id: self.task_id,
          task_status: TaskStatus::Running,
          events: new_msgs,
        }
      }
      false => TaskState {
        task_id: self.task_id,
        task_status: TaskStatus::Finished,
        events: vec![format!("Task {} has finished running", self.task_id)],
      },
    }
  }

  pub fn new(guid: Uuid) -> TaskState {
    TaskState {
      task_id: guid,
      task_status: TaskStatus::Queued,
      events: vec![],
    }
  }
}

/// An enumeration used to create tasks.
pub trait LongRunnerRouter: Sized + Clone + std::fmt::Debug + Sync + Send {
  /// Convert from a string given in the URL into the router type
  fn from_str(action: String) -> Result<Self, String>;

  /// Generate a new task based on the payload.
  fn to_task(&self) -> Result<LongRunnerTask<Self>, String>;
}

#[derive(Debug, Clone)]
pub struct LongRunnerTask<R>
where
  R: LongRunnerRouter + std::fmt::Debug + Sync + Send,
{
  pub task_id: Uuid,
  pub task_route: R,
  pub task_params: String,
  pub state: Arc<RwLock<TaskState>>,
}

/// A simple record to tell the longrunner which call to run with a given payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongRunnerRun<R> {
  pub route: R,
}
