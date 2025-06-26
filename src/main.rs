#![warn(clippy::all, rust_2018_idioms)]

// native:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        horizon::APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(horizon::WeatherApp::new(cc)))),
    )
}
