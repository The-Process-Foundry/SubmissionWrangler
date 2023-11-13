//! The root of the Tauri Desktop app
//!
//! All the async was based on the code written at https://rfdonnelly.github.io/posts/tauri-async-rust-process/

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod graph_db;

use tauri::Manager;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber;

struct AsyncProcInputTx {
  inner: Mutex<mpsc::Sender<String>>,
}

fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
  info!(?message, "Replying using rs2js:");
  manager
    .emit_all("rs2js", format!("rs: {}", message))
    .unwrap();
}

/// Receive a message from the client and spawns
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

      tauri::async_runtime::spawn(async move {
        async_process_model(async_proc_input_rx, async_proc_output_tx).await
      });

      let app_handle = app.handle();
      tauri::async_runtime::spawn(async move {
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
