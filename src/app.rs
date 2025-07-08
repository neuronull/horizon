use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use tokio::runtime::{Builder, Runtime};

use eframe::{CreationContext, Frame};
use egui::{Context, Label, Layout, ScrollArea, TextEdit, Ui};

use lib_weather::fetch_forecast;

#[derive(Deserialize, Serialize, Default)]
pub struct AppState {
    pub fetch_requested: bool,
    // TODO: find a way to not keep both types. fields are not editable without this.
    latitude_str: String,
    longitude_str: String,
    latitude: f64,
    longitude: f64,
    // widgets: Widgets,
}

pub struct AppController {
    runtime: Runtime,
    state: Arc<Mutex<AppState>>,
}

impl AppController {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to build runtime");

        let state = Arc::new(Mutex::new(AppState::new(cc)));

        Self { runtime, state }
    }
}

impl eframe::App for AppController {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let state = self.state.lock().unwrap();

        eframe::set_value(storage, eframe::APP_KEY, &*state);
    }

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let mut state = self.state.lock().unwrap();
        if state.fetch_requested {
            state.fetch_requested = false; // reset flag

            let latitude = state.latitude;
            let longitude = state.longitude;

            self.runtime.spawn(async move {
                let api_key = std::env!("PIRATEWEATHER_API_KEY");
                match fetch_forecast(&api_key, latitude, longitude).await {
                    Ok(resp) => println!("{resp:#?}"),
                    Err(err) => eprintln!("{err}"),
                }
            });
        }

        state.update(ctx, frame);
    }
}

impl AppState {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn update_location(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Location");
        });
        ui.separator();

        egui::Grid::new("location_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.add(Label::new("Latitude: "));

                // latitude
                let latitude_response =
                    ui.add(TextEdit::singleline(&mut self.latitude_str).hint_text("37.233"));
                ui.end_row();

                // longitude
                ui.add(Label::new("Longitude: "));
                let longitude_response =
                    ui.add(TextEdit::singleline(&mut self.longitude_str).hint_text("-115.800"));

                if latitude_response.changed() || longitude_response.changed() {}
                ui.end_row();
            });

        egui::ScrollArea::vertical().show(ui, |ui| {
            if ui.button("Fetch").clicked() {
                // validate lat and lon
                // TODO surface error as modal
                let (lat, lon) =
                    validate_lat_long_input(&self.latitude_str, &self.longitude_str).unwrap();
                self.latitude = lat;
                self.longitude = lon;
                self.fetch_requested = true;
            }
        });
        ui.separator();
    }

    fn update_widget_toggle_pane(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Widgets");

            ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |_ui| {
                    // display widget labels
                });
            });
        });
        ui.separator();
    }
}

impl eframe::App for AppState {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .default_width(100.0)
            .min_width(100.0)
            .show(ctx, |ui| {
                self.update_location(ui);

                self.update_widget_toggle_pane(ui);
            });

        egui::CentralPanel::default().show(ctx, |_ui| {
            // widget display area
        });
    }
}

// ensures input string is a valid float and clears the buffer if not
fn validate_lat_long_input(lat: &str, lon: &str) -> Result<(f64, f64), String> {
    let validate = |input: &str| -> Result<f64, String> {
        input
            .parse::<f64>()
            .map_err(|_| "Invalid input: number not parseable as float.".to_string())
    };
    Ok((validate(lat)?, validate(lon)?))
}
