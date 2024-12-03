//! The root of the Tauri Desktop app
//!
//! All the async was based on the code written at https://rfdonnelly.github.io/posts/tauri-async-rust-process/

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, Mutex};
use tracing::{info, warn};
use uuid::Uuid;

use wrangler_common::{
  calls::*,
  longrunner::{LongRunnerRouter, LongRunnerRun},
};
use wrangler_server::prelude::*;

struct State {
  input_channel: Mutex<mpsc::Sender<String>>,
  server: Arc<Server>,
}

fn rs2js(message: String, app: &AppHandle) {
  info!(?message, "Replying using server_reply event:");
  app.emit("server_reply", message).unwrap();
}

/// Receive a message from the client and forwards it along to the server side. Messages are passed
/// along serialized, leaving it to the server to fully process them.
#[tauri::command]
async fn call_server(args: String, state: tauri::State<'_, State>) -> Result<(), String> {
  info!(?args, "Received tauri::command: call_server");

  // Send it to the server
  let async_proc_input_tx = state.input_channel.lock().await;

  // Forward the message along to the listener
  async_proc_input_tx.send(args).await.map_err(|e| {
    let msg = e.to_string();
    warn!("call_server - error with input_channel lock:\n\t{}", e);
    msg
  })
}

/// Send a new task to the LongRunner to be queued up.
#[tauri::command]
async fn run(args: String, state: tauri::State<'_, State>) -> Result<String, String> {
  info!(?args, "Received call_run");
  let guid: Uuid;

  // Deserialize Args
  let args: Result<LongRunnerRun<LongRunnerCall>, _> = serde_json::from_str(&args);
  match args {
    Ok(call) => {
      let task = call.route.to_task().unwrap();
      guid = task.task_id;
      let server: Arc<Server> = state.server.clone();
      server.long_runner.enqueue(task);
      Ok(())
    }
    Err(err) => {
      guid = Uuid::nil();
      Err(format!("Deserialization error: {}", err))
    }
  }?;

  Ok(guid.to_string())
}

/// An asynchronous loop to listen for new messages. When one is received, is processes it via the
/// handler.
async fn listen(
  mut input_rx: mpsc::Receiver<String>,
  output_tx: mpsc::Sender<String>,
  server: Arc<Server>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  while let Some(input) = input_rx.recv().await {
    if input == "Halt" {
      warn!("App Listener received a halt command. Shutting down now");
      break;
    }
    info!("App.listener received a message: {}", input);
    let output = server.handle(input).await;
    info!("App.listener is returning message: {}", output);
    output_tx.send(output).await?;
  }

  Ok(())
}

fn main() {
  tracing_subscriber::fmt::init();

  // Create sockets for communicating between the client and server
  let (input_sender, input_receiver) = mpsc::channel(1);
  let (output_sender, mut output_receiver) = mpsc::channel(1);

  // Initialize a singleton server
  let server = Arc::new(Server::create());

  // Integrate with tokio: https://rfdonnelly.github.io/posts/tauri-async-rust-process/
  tauri::Builder::default()
    .manage(State {
      input_channel: Mutex::new(input_sender),
      server: server.clone(),
    })
    .setup(|app| {
      // Automatically open the chrome dev-tools when building locally
      #[cfg(debug_assertions)]
      {
        // let window = app.get_window("main").unwrap();
        // window.open_devtools();
        // window.close_devtools();
      }

      // Kick off the listener
      tauri::async_runtime::spawn(
        async move { listen(input_receiver, output_sender, server).await },
      );

      // tauri::async_runtime::spawn(async move {
      //   async_process_model(async_proc_input_rx, async_proc_output_tx).await
      // });

      // Return the processed event to the frontend
      let app_handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
        loop {
          if let Some(output) = output_receiver.recv().await {
            rs2js(output, &app_handle);
          }
        }
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![call_server, run])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
