use std::{collections::HashMap, rc::Rc};

use tracing::info;
use wasm_bindgen_futures::{future_to_promise, spawn_local};
use yew::prelude::*;

pub(crate) mod glue;
mod views;
pub use views::organization::*;
use wrangler_common::longrunner::LongRunnerRun;

/// Messages that can be sent to the server for processing
#[derive(Debug, Clone)]
enum Call {
  Ping,
  LoadCSV,
  QueryAll,
}

/// Root pages to be displayed in the body of the page
#[derive(Debug, Clone)]
enum PageView {
  Dashboard,
  Organizations,
}

/// Messages that have side effects that may eventually alter the AppState
#[derive(Debug, Clone)]
enum AppAction {
  /// Send a message to the server
  CallServer(Call),

  /// A message that an asynchronous call has been completed and the result should be processed
  Thunk(String),

  /// Switch the page
  ChangePage(PageView),
}

/// Application configuration settings and current runtime values of the app
#[derive(Debug, Clone)]
struct AppState {
  // /// User settings for how to display the App
  // settings: String,
  /// This is the local data cache as stored by Grapht. This will act as a local database and hide
  /// how the sausage is made.
  data_graph: String,

  /// The active page displayed in the body
  current_page: PageView,
}

impl AppState {
  /// Perform an update on the state based on the info contained in the call
  pub fn call(self, call: Call) -> Self {
    let new_state = match call {
      Call::Ping => self,
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
      let call_str = format!("\"{:?}\"", call);
      info!("Spawned the call with a thunk: {:?}", call_str);
      let mut args: HashMap<&str, String> = HashMap::new();
      args.insert("args", call_str);
      let called = glue::invoke("call_server", serde_wasm_bindgen::to_value(&args).unwrap()).await;
      info!("Server replied with: '{:#?}'", called);
      let result = AppAction::Thunk(format!("Received a call result: {:?}", called));
      // let result = match called {
      //   Ok(result) => AppAction::Thunk(format!("Received a call result: {:?}", result)),
      //   Err(err) => AppAction::Thunk(format!("Call failed in the end with error: {:?}", err)),
      // };
      info!("Completed call. Sending result: {:?}", result);
    });

    new_state
  }

  pub fn change_page(self, page: PageView) -> Self {
    AppState {
      current_page: page,
      ..self
    }
  }
}

impl Default for AppState {
  fn default() -> AppState {
    AppState {
      // settings: "No Settings Yet".to_string(),
      data_graph: "Initialized".to_string(),
      current_page: PageView::Organizations,
    }
  }
}

impl Reducible for AppState {
  type Action = AppAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    let current_state = self.as_ref().clone();
    let new_state = match action {
      AppAction::CallServer(call) => current_state.call(call),
      AppAction::ChangePage(new_body) => current_state.change_page(new_body),
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
  let state = use_reducer(AppState::default);

  let clicked = |call: Call| {
    let state = state.clone();

    Callback::from(move |_e: MouseEvent| {
      let call = call.clone();
      info!("Clicked button for {:?}", call);
      state.dispatch(AppAction::CallServer(call));
    })
  };

  let greet_input_ref = use_node_ref();
  let ping_guid = use_state(|| None);
  let has_pinged = use_state(|| false);

  info!("Rendering the app");
  {
    let ping_guid = ping_guid.clone();
    let ping_guid2 = ping_guid.is_none() && *has_pinged;
    use_effect_with(ping_guid2, move |_| {
      spawn_local(async move {
        if ping_guid2 {
          info!("Ping info is currently nil");
          let mut args: HashMap<&str, String> = HashMap::new();
          let runner = LongRunnerRun { route: "ImportCSV" };
          args.insert("args", serde_json::to_string(&runner).unwrap());
          let args = serde_wasm_bindgen::to_value(&args).unwrap();
          info!("Sending args: {:?}", args);
          let new_msg = glue::invoke("call_run", args).await.as_string().unwrap();
          info!("Received response message: {}", new_msg);
          ping_guid.set(Some(uuid::Uuid::parse_str(&new_msg).unwrap()));
        }
      })
    });
  }

  let submitted = {
    let has_pinged = has_pinged.clone();

    Callback::from(move |e: SubmitEvent| {
      e.prevent_default();
      info!("Clicked the submit button");
      has_pinged.set(true);
    })
  };

  let body = match &state.current_page {
    PageView::Dashboard => todo!("No dashboard yet"),
    PageView::Organizations => html! {<OrgGrid></OrgGrid>},
  };

  html! {
    <div style="width: 100%;">
      <h1>{ "Welcome to the Submission Wrangler" }</h1>
      <hr />
      {body}
      <hr />
      <div style="width: 100%;">
        <div style="width: 50%; padding: 6px; display: inline;">
          <button onclick={clicked(Call::Ping)}>{"Ping"}</button>
        </div>
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
      <div><br /><hr /><br /></div>
      <div>
        <h1>{"Ping as state"}</h1>

        <form class="row" onsubmit={submitted}>
            <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
            <button type="submit">{"Greet"}</button>
        </form>
        <p>{ format!("Ping Guid: {:?}.", &ping_guid) }</p>
      </div>
    </div>
  }
}

fn main() {
  // Enable Console.log for displaying tracing messages before anything else
  logger::init();

  yew::Renderer::<App>::new().render();
}
