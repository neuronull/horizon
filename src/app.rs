use std::{
    collections::BTreeSet,
    marker::{PhantomData, Send},
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use eframe::{CreationContext, Frame};
use egui::{Context as Ctx, Id, Label, Layout, Modal, ScrollArea, TextEdit, Ui};
use tokio::runtime::{Builder, Runtime};

use super::Widgets;
use lib_weather::{WeatherData, WeatherFetch};

/// State machine for fetching weather data
#[derive(Default, PartialEq)]
pub enum FetchState {
    /// The fetch request has completed.
    #[default]
    Completed,
    /// The fetch has been requested, but has not started yet.
    Requested,
    /// The fetch is in progress.
    InProgress,
}

#[derive(Default)]
pub struct AppState {
    pub fetch_state: FetchState,
    // TODO: find a way to not keep both types. fields are not editable without this.
    latitude_str: String,
    longitude_str: String,
    latitude: f64,
    longitude: f64,
    location_error_modal_open: bool,
    widgets: Widgets,
    open_widgets: BTreeSet<String>,
}

pub struct AppController<D, F>
where
    D: WeatherData + Default + Send + 'static,
    F: WeatherFetch<Output = D> + Send,
{
    runtime: Runtime,
    state: Arc<Mutex<AppState>>,
    /// Weather data
    pub data: Arc<Mutex<D>>,
    _fetcher: PhantomData<F>,
}

impl<D, F> AppController<D, F>
where
    D: WeatherData + Default + Send + 'static,
    F: WeatherFetch<Output = D> + Send,
{
    /// # Panics
    ///
    /// Will panic if tokio runtime build fails
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to build runtime");

        let data = Arc::new(Mutex::new(D::default()));
        let state = Arc::new(Mutex::new(AppState::new(cc)));

        Self {
            runtime,
            state,
            data,
            _fetcher: PhantomData,
        }
    }

    fn fetch(&mut self, lat: f64, lon: f64) {
        let data = Arc::clone(&self.data);
        let state = Arc::clone(&self.state);

        self.runtime.spawn(async move {
            println!("doing fetch");
            // TODO surface error to user
            match F::fetch_weather(lat, lon).await {
                Ok(response) => {
                    let mut data = data.lock().unwrap();
                    *data = response;
                    let mut state = state.lock().unwrap();
                    state.fetch_state = FetchState::Completed;
                }
                Err(err) => eprintln!("{err}"),
            }
        });
    }
}

impl<D, F> eframe::App for AppController<D, F>
where
    D: WeatherData + Default + Send + 'static,
    F: WeatherFetch<Output = D> + Send,
{
    // TODO: re-enable for feature to save state
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     let state = self.state.lock().unwrap();

    //     eframe::set_value(storage, eframe::APP_KEY, &*state);
    // }

    fn update(&mut self, ctx: &Ctx, frame: &mut Frame) {
        let mut requested = false;
        let mut lat = 0.0;
        let mut lon = 0.0;
        {
            let mut state = self.state.lock().unwrap();
            if state.fetch_state == FetchState::Requested {
                requested = true;
                lat = state.latitude;
                lon = state.longitude;
                state.fetch_state = FetchState::InProgress;
            }
        }

        if requested {
            self.fetch(lat, lon);
        }

        let data = self.data.lock().unwrap();
        let mut state = self.state.lock().unwrap();
        state.update(&*data, ctx, frame);
    }
}

impl AppState {
    /// Called once before the first frame.
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        // TODO: re-enable for feature to save state
        // Load previous app state (if any).
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Self {
            widgets: Widgets::new(),
            ..Default::default()
        }
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
                ui.add(TextEdit::singleline(&mut self.latitude_str).hint_text("37.233"));
                ui.end_row();

                // longitude
                ui.add(Label::new("Longitude: "));
                ui.add(TextEdit::singleline(&mut self.longitude_str).hint_text("-115.800"));

                ui.end_row();
            });

        egui::ScrollArea::vertical().show(ui, |ui| {
            if ui.button("Fetch").clicked() && self.fetch_state == FetchState::Completed {
                // validate lat and lon
                match validate_lat_lon_input(&self.latitude_str, &self.longitude_str) {
                    Ok((lat, lon)) => {
                        self.latitude = lat;
                        self.longitude = lon;
                        self.fetch_state = FetchState::Requested;
                    }
                    Err(_err) => {
                        self.location_error_modal_open = true;
                    }
                }
            }
        });
        ui.separator();
    }

    // display widget selectors
    fn update_widget_toggle_pane(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Widgets");

            ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    self.widgets.checkboxes(ui, &mut self.open_widgets);
                });
            });
        });
        ui.separator();
    }

    fn show_location_error_modal(&mut self, ui: &mut Ui) {
        Modal::new(Id::new("location_error_modal")).show(ui.ctx(), |ui| {
            ui.set_width(200.0);
            ui.heading("Location invalid.");

            ui.add_space(32.0);

            egui::Sides::new().show(
                ui,
                |_ui| {},
                |ui| {
                    if ui.button("close").clicked() {
                        self.location_error_modal_open = false;
                    }
                },
            );
        });
    }

    fn update<D: WeatherData>(&mut self, data: &D, ctx: &Ctx, _frame: &mut Frame) {
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .default_width(200.0)
            .min_width(200.0)
            .show(ctx, |ui| {
                self.update_location(ui);

                self.update_widget_toggle_pane(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // widget display area
            self.widgets.windows(ctx, &mut self.open_widgets, data);

            if self.location_error_modal_open {
                self.show_location_error_modal(ui);
            }
        });
    }
}

// ensures input string is a valid float and clears the buffer if not
fn validate_lat_lon_input(lat: &str, lon: &str) -> Result<(f64, f64)> {
    let validate = |input: &str| -> Result<f64> {
        input
            .parse::<f64>()
            .context("Invalid input: number not parseable as float.")
    };
    Ok((validate(lat)?, validate(lon)?))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_lat_lon_happy() {
        assert_eq!(
            (37.233, -115.800),
            validate_lat_lon_input("37.233", "-115.800").unwrap()
        );
        assert_eq!(
            (37.0, -115.0),
            validate_lat_lon_input("37", "-115").unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "Invalid input: number not parseable as float.")]
    fn validate_lat_lon_fail_empty() {
        validate_lat_lon_input("", "").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid input: number not parseable as float.")]
    fn validate_lat_lon_fail_nonnumber_lat() {
        validate_lat_lon_input("area", "-115").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid input: number not parseable as float.")]
    fn validate_lat_lon_fail_nonnumber_lon() {
        validate_lat_lon_input("37", "fiftyone").unwrap();
    }
}
