use std::rc::Rc;

use tracing::info;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub(crate) mod glue;

/// Messages that can be sent to the server for processing
#[derive(Debug, Clone)]
enum Call {
  LoadCSV,
  QueryAll,
}

/// Messages that have side effects that may eventually alter the AppState
#[derive(Debug, Clone)]
enum AppAction {
  /// Send a message to the server
  CallServer(Call),

  /// A message that an asynchronous call has been completed and the result should be processed
  Thunk(String),
}

/// Application configuration settings and current runtime values of the app
#[derive(Debug, Clone)]
struct AppState {
  // /// User settings for how to display the App
  // settings: String,
  /// This is the local data cache as stored by Grapht. This will act as a local database and hide
  /// how the sausage is made.
  data_graph: String,
}

impl AppState {
  pub fn call(self, call: Call) -> Self {
    let new_state = match call {
      Call::LoadCSV => AppState {
        data_graph: "Loading CSV ...".to_string(),
        ..self
      },
      Call::QueryAll => AppState {
        data_graph: "Querying All ...".to_string(),
        ..self
      },
    };

    spawn_local(async move {
      let call_str = format!("{:?}", call);
      info!("Spawned the call with a thunk: {:?}", call_str);
      let result = match glue::call_server(call_str).await {
        Ok(result) => AppAction::Thunk(format!("Received a call result: {:?}", result)),
        Err(err) => AppAction::Thunk(format!("Call failed in the end with error: {:?}", err)),
      };
      info!("Completed call. Sending thunk: {:?}", result);
    });

    new_state
  }
}

impl Default for AppState {
  fn default() -> AppState {
    AppState {
      // settings: "No Settings Yet".to_string(),
      data_graph: "Initialized".to_string(),
    }
  }
}

impl Reducible for AppState {
  type Action = AppAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    info!("Running the AppState Reducer for action: {:?}", action);

    let current_state = self.as_ref().clone();
    let new_state = match action {
      AppAction::CallServer(call) => current_state.call(call),
      AppAction::Thunk(msg) => {
        info!("Processing an async result: {:?}", msg);
        current_state
      }
    };
    new_state.into()
  }
}

/// Enable tracing to dump to the console
mod logger {
  use tracing_subscriber::{
    filter::filter_fn,
    fmt::format::{FmtSpan, Pretty},
    prelude::*,
  };
  use tracing_web::{performance_layer, MakeConsoleWriter};

  pub(crate) fn init() {
    let fmt_layer = tracing_subscriber::fmt::layer()
      .with_ansi(false)
      .without_time()
      .with_writer(MakeConsoleWriter)
      .with_span_events(FmtSpan::ACTIVE)
      .with_filter(filter_fn(|metadata| metadata.target() == "fhl_client"));

    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
      .with(fmt_layer)
      .with(perf_layer)
      .init();
  }
}

/// The application root
#[function_component(App)]
fn app() -> Html {
  info!("Initializing the AppState");
  let state = use_reducer(AppState::default);

  let clicked = |call: Call| {
    let state = state.clone();
    info!("Initializing the callback for call: {:?}", call);

    Callback::from(move |_e: MouseEvent| {
      let call = call.clone();
      info!("Clicked button for {:?}", call);
      state.dispatch(AppAction::CallServer(call));
    })
  };

  info!("Rendering the App");
  html! {
    <div style="width: 100%;">
      <h1>{ "Welcome to the Submission Wrangler" }</h1>
      <hr />
      <div style="width: 100%;">
        <div style="width: 50%; padding: 6px; display: inline;">
          <button onclick={clicked(Call::LoadCSV)}>{"Load CSV"}</button>
        </div>
        <div style="width: 50%; padding: 6px; display: inline;">
          <button onclick={clicked(Call::QueryAll)}>{"Query All"}</button>
        </div>
      </div>
      <div>

        <table style="border: 2px; border-color: white;">
          <tr>
            <td>{"Data State"}</td>
            <td>
              {state.data_graph.clone()}
            </td>
          </tr>
        </table>
      </div>
    </div>
  }
}

fn main() {
  // Enable Console.log for displaying tracing messages before anything else
  logger::init();

  yew::Renderer::<App>::new().render();
}
