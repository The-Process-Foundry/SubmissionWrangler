use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use tokio::task::spawn;
use tracing::{error, info};
use uuid::Uuid;

use crate::workspace::Workspace;
use wrangler_common::longrunner::{LongRunnerRouter, LongRunnerTask, TaskStatus};

pub struct LongRunner<R>
where
  R: LongRunnerRouter,
{
  /// An ordered list of FIFO tasks.
  queue: Arc<RwLock<VecDeque<Arc<RwLock<LongRunnerTask<R>>>>>>,
  /// The tasks that are currently active
  running: Arc<RwLock<HashMap<Uuid, Arc<RwLock<LongRunnerTask<R>>>>>>,
  /// A history of all the tasks, including queued.
  tasks: Arc<RwLock<HashMap<Uuid, Arc<RwLock<LongRunnerTask<R>>>>>>,
  /// A function used to send the task to the correct runner.
  executor: Arc<dyn Fn(Arc<RwLock<LongRunnerTask<R>>>, Workspace) + Send + Sync>,
  /// A server context to be made available to each tasks
  context: Workspace,
}

impl<R> LongRunner<R>
where
  R: LongRunnerRouter + 'static,
{
  /// Add a task to the queue as ready for running
  pub fn enqueue(&self, task: LongRunnerTask<R>) {
    let mut tasks = self.tasks.write().unwrap();
    let wrapped = Arc::new(RwLock::new(task.clone()));
    tasks.insert(task.task_id, wrapped.clone());
    drop(tasks);

    let mut queue = self.queue.write().unwrap();
    queue.push_back(wrapped);
    drop(queue);

    self.start_tasks();
  }

  /// Retrieve the first task from the queue, removing it from the list.
  fn dequeue(&self) -> Arc<RwLock<LongRunnerTask<R>>> {
    let mut queue = self.queue.write().unwrap();
    // This should always be Some because the length is tested before running this function
    queue.pop_front().unwrap()
  }

  /// Retrieve a task from the full history of tasks. This is a memory leak waiting to happen, so
  /// once the event history drained the task is finished, it should be removed from the tasks list.
  pub fn get_task(&self, guid: Uuid) -> Option<Arc<RwLock<LongRunnerTask<R>>>> {
    let tasks = self.tasks.read().unwrap();
    match tasks.get(&guid) {
      Some(task) => {
        let inner_state = task.read().unwrap().state.clone();
        let mut state = inner_state.write().unwrap();
        match state.task_status {
          TaskStatus::Paused => state.task_status = TaskStatus::Running,
          TaskStatus::Running => state.task_status = TaskStatus::Finished,
          _ => (),
        }
        Some(task.clone())
      }
      None => None,
    }
  }

  pub fn queue_length(&self) -> usize {
    let queue = self.queue.read().unwrap();
    queue.len()
  }

  /// Execute a task on a new thread.
  fn start_task(&self, task: Arc<RwLock<LongRunnerTask<R>>>) {
    let runner = self.executor.clone();

    // Change the task state to Running
    info!("Reading the task");
    let reader = task.read().unwrap();
    info!("Writing the state");
    let mut state = reader.state.write().unwrap();
    info!("Updated the state to Paused");
    state.task_status = TaskStatus::Paused;
    let task_id = reader.task_id.clone();
    drop(state);

    info!("Dropped state in start");
    let task = task.clone();
    let running = self.running.clone();
    let ctx = self.context.clone();

    spawn(async move {
      // Send the task to the proper runner.
      (runner.clone())(task.clone(), ctx);

      // Cleanup after the task.
      // Remove the task from the running hash.
      let mut running = running.write().unwrap();
      match running.remove(&task_id) {
        Some(_task) => info!("Finished task {}", task_id),
        None => {
          error!("Couldn't find {} in the running tasks", task_id);
          panic!("Couldn't find {} in the running tasks", task_id)
        }
      }

      //
      let reader = task.read().unwrap();
      let mut state = reader.state.write().unwrap();
      match state.task_status {
        TaskStatus::Running => state.task_status = TaskStatus::Finished,
        _ => (),
      }
    });
  }

  /// Begin as many tasks as there are threads allocated.
  pub fn start_tasks(&self) {
    // check the queue
    while self.queue_length() > 0 {
      let task = self.dequeue();
      // Change the task state to Paused, as it's not running yet.
      let reader = task.read().unwrap();
      let guid = reader.task_id;
      let mut state = reader.state.write().unwrap();
      state.task_status = TaskStatus::Paused;
      drop(state);

      // Add the task to the "running" hashmap
      let mut running = self.running.write().unwrap();
      match running.insert(guid.clone(), task.clone()) {
        Some(_old) => {
          panic!("Received repeat guid {}", guid)
        }
        None => (),
      }
      drop(running);

      self.start_task(task.clone());
    }
  }

  pub fn new(
    executor: Arc<dyn Fn(Arc<RwLock<LongRunnerTask<R>>>, Workspace) + Send + Sync>,
    context: Workspace,
  ) -> LongRunner<R> {
    {
      LongRunner {
        queue: Arc::new(RwLock::new(VecDeque::from([]))),
        running: Arc::new(RwLock::new(HashMap::new())),
        tasks: Arc::new(RwLock::new(HashMap::default())),
        executor,
        context,
      }
    }
  }
}
