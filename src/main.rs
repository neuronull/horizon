#![warn(clippy::pedantic)]
#![warn(clippy::all, rust_2018_idioms)]

use tokio::sync::mpsc::{self, Receiver};

use horizon::{AppController, AppState};
use lib_weather::{PirateData, PirateWeather};

fn init_logging() -> Receiver<String> {
    let (logtx, logrx) = mpsc::channel::<String>(100);
    horizon::setup_logging(logtx);
    logrx
}

// native:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use tokio::runtime::Builder;

    let logrx = init_logging();

    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build runtime");

    let state = AppState::new(logrx, runtime.handle());

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        horizon::APP_NAME,
        native_options,
        Box::new(|_cc| {
            Ok(Box::new(AppController::<PirateData, PirateWeather>::new(
                state,
                Some(runtime),
            )))
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    let logrx = init_logging();
    let state = AppState::new(logrx);

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| {
                    Ok(Box::new(AppController::<PirateData, PirateWeather>::new(
                        state, None,
                    )))
                }),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
