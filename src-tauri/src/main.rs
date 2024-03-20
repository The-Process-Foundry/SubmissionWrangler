//! The root of the Tauri Desktop app
//!
//! All the async was based on the code written at https://rfdonnelly.github.io/posts/tauri-async-rust-process/

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tracing::info;

mod graph_db;
use graph_db::{Neo4jConfig, Neo4jConnection};

// Create a connection to the Neo4j server

struct AsyncProcInputTx {
  inner: Mutex<mpsc::Sender<String>>,
}

fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
  info!(?message, "Replying using rs2js:");
  manager
    .emit_all("rs2js", format!("rs: {}", message))
    .unwrap();
}

/// Receive a message from the client and forwards it along to the server side
#[tauri::command]
async fn call_server(
  message: String,
  state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), String> {
  info!(?message, "Received tauri::command: call_server");
  let async_proc_input_tx = state.inner.lock().await;
  async_proc_input_tx
    .send(message)
    .await
    .map_err(|e| e.to_string())
}

async fn async_process_model(
  mut input_rx: mpsc::Receiver<String>,
  output_tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  while let Some(input) = input_rx.recv().await {
    let output = input;
    output_tx.send(output).await?;
  }

  Ok(())
}

// Connect to the Neo4j database
async fn db_connect() -> core::result::Result<Neo4jConnection, String> {
  // A singleton workspace shared by the entire Tauri App
  // let workspace = Workspace::init(WorkspaceConfig::Default());
  let graph_config = Neo4jConfig {
    uri: "localhost:7687".to_string(),
    username: "neo4j".to_string(),
    password: "neo_pass".to_string(),
  };

  let conn = Neo4jConnection::connect(graph_config).await?;
  conn.ping().await?;

  // Ping the connection to ensure it works
  Ok(conn)
}

fn main() {
  tracing_subscriber::fmt::init();

  let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
  let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

  // Integrate with tokio: https://rfdonnelly.github.io/posts/tauri-async-rust-process/
  tauri::Builder::default()
    .manage(AsyncProcInputTx {
      inner: Mutex::new(async_proc_input_tx),
    })
    .setup(|app| {
      // Automatically open the chrome dev-tools when building locally
      #[cfg(debug_assertions)]
      {
        let window = app.get_window("main").unwrap();
        window.open_devtools();
        window.close_devtools();
      }

      // Listen for
      tauri::async_runtime::spawn(async move {
        async_process_model(async_proc_input_rx, async_proc_output_tx).await
      });

      // Return the processed event to the frontend
      let app_handle = app.handle();
      tauri::async_runtime::spawn(async move {
        let _db_conn = db_connect().await.unwrap();

        loop {
          if let Some(output) = async_proc_output_rx.recv().await {
            rs2js(output, &app_handle);
          }
        }
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![call_server])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
