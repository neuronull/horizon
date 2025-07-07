use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use eframe::{CreationContext, Frame};
use egui::{Context, Label, Layout, ScrollArea, TextEdit};

use lib_weather::fetch_forecast;

#[derive(Deserialize, Serialize, Default)]
pub struct WeatherApp {
    latitude: String,
    longitude: String,
    // widgets: Widgets,
}

impl WeatherApp {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for WeatherApp {
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
                            ui.add(TextEdit::singleline(&mut self.latitude).hint_text("37.233"));
                        if latitude_response.changed() {
                            //
                        }
                        ui.end_row();

                        // longitude
                        ui.add(Label::new("Longitude: "));
                        let longitude_response =
                            ui.add(TextEdit::singleline(&mut self.longitude).hint_text("-115.800"));
                        if longitude_response.changed() {
                            //
                        }
                        ui.end_row();
                    });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    if ui.button("Fetch").clicked() {
                        // validate lat and lon
                        // TODO surface error as modal
                        let (lat, lon) =
                            validate_lat_long_input(&mut self.latitude, &mut self.longitude)
                                .unwrap();

                        let rt = Runtime::new().unwrap();
                        let api_key = std::env!("PIRATEWEATHER_API_KEY");
                        let result = rt.block_on(fetch_forecast(&api_key, lat, lon));

                        if let Ok(data) = result {
                            println!("{:#?}", data);
                        } else if let Err(err) = result {
                            // TODO surface as modal
                            println!("{err}");
                        }
                    }
                });
                ui.separator();

                // widget toggle area
                ui.vertical_centered(|ui| {
                    ui.heading("Widgets");

                    ScrollArea::vertical().show(ui, |ui| {
                        ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
                            // display widget labels
                        });
                    });
                });
                ui.separator();
            });

        egui::CentralPanel::default().show(ctx, |_ui| {
            // widget display area
        });
    }
}

// ensures input string is a valid float and clears the buffer if not
fn validate_lat_long_input(lat: &mut String, lon: &mut String) -> Result<(f64, f64), String> {
    let validate = |input: &mut String| -> Result<f64, String> {
        input
            .parse::<f64>()
            .map_err(|_| "Invalid input: number not parseable as float.".to_string())
    };
    Ok((validate(lat)?, validate(lon)?))
}
