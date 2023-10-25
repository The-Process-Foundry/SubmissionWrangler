use tracing::info;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;
// use web_sys::window;

use yew::prelude::*;

#[derive(Debug, Clone)]
enum Call {
  LoadCSV,
  QueryAll,
}

// Enable tracing to dump to the console
mod logger {
  use tracing_subscriber::{
    fmt::format::{FmtSpan, Pretty},
    prelude::*,
  };
  use tracing_web::{performance_layer, MakeConsoleWriter};

  pub(crate) fn init() {
    let fmt_layer = tracing_subscriber::fmt::layer()
      .with_ansi(false)
      .without_time()
      .with_writer(MakeConsoleWriter)
      .with_span_events(FmtSpan::ACTIVE);
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
      .with(fmt_layer)
      .with(perf_layer)
      .init();
  }
}
#[function_component(App)]
fn app() -> Html {
  // Enable Console.log
  logger::init();

  info!("Rendering the App");
  fn clicked(call: Call) -> Callback<MouseEvent> {
    info!("Initializing the callback: {:?}", call);
    Callback::from(move |_e: MouseEvent| info!("Clicked button for {:?}", call))
  }

  html! {
    <div style="width: 100%;">
      <h1>{ "Welcome to the Submission Wrangler" }</h1>
      <hr />
      <div style="width: 100%;">

        <div style="width: 50%; padding: 6px; display: inline;">
          <button onclick={clicked(Call::LoadCSV)}>{"Load CSV"}</button>
        </div>
        <div style="width: 50%; padding: 6px; display: inline;">
          <button onclick={clicked(Call::QueryAll)}>{"Query All Orgs"}</button>
        </div>
      </div>
    </div>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
