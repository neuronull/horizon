use std::{
    collections::BTreeSet,
    marker::{PhantomData, Send},
};

use anyhow::{Context, Result};
use eframe::{CreationContext, Frame};
use egui::{Context as Ctx, Id, Label, Layout, Modal, ScrollArea, TextEdit, Ui};
use egui_extras::syntax_highlighting;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::watch::{Receiver, Sender};
use tracing::{error, info};

use super::{setup_logging, Logs, Widgets};
use lib_weather::{WeatherData, WeatherFetch};

const A51_LAT: &str = "37.233";
const A51_LON: &str = "-115.800";

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

#[derive(PartialEq, Default)]
enum View {
    #[default]
    Weather,
    Log,
}

#[derive(Default)]
pub struct AppState {
    // UI view state
    current_view: View,
    log_view_selected: bool,
    weather_view_selected: bool,
    /// location latitude
    latitude_str: String,
    /// location longitude
    longitude_str: String,
    location_error_modal_open: bool,
    /// UI widgets
    widgets: Widgets,
    open_widgets: BTreeSet<String>,
}

/// Manages intersection between the UI state and weather data.
pub struct AppController<D, F>
where
    D: WeatherData + Default + Sync + Send + 'static,
    F: WeatherFetch<Output = D> + Send,
{
    sender: Sender<D>,
    receiver: Receiver<D>,
    runtime: Runtime,
    state: AppState,
    logs: Logs,
    /// Weather data
    pub data: D,
    _fetcher: PhantomData<F>,
    /// state of the weather data fetch operation
    fetch_state: FetchState,
}

impl<D, F> AppController<D, F>
where
    D: WeatherData + Default + Sync + Send + 'static,
    F: WeatherFetch<Output = D> + Send,
{
    /// # Panics
    ///
    /// Will panic if tokio runtime build fails
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build runtime");

        let data = D::default();
        let state = AppState::new(cc);
        let (sender, receiver) = tokio::sync::watch::channel(D::default());

        let logs = setup_logging();
        info!("Initializing app");

        let mut controller = Self {
            sender,
            receiver,
            runtime,
            state,
            data,
            logs,
            fetch_state: FetchState::default(),
            _fetcher: PhantomData,
        };

        controller.fetch();

        controller
    }

    fn fetch(&mut self) {
        // validate lat and lon first.
        // we could do this in the UI update area when fetch button clicked,
        // but then we'd either have to store a second set of vars for the parsed
        // float values, or re-parse them here. Neither of which are ideal.
        let (lat, lon) =
            match validate_lat_lon_input(&self.state.latitude_str, &self.state.longitude_str) {
                Ok(res) => res,
                Err(err) => {
                    error!("Invalid location submitted: {err}");
                    self.state.location_error_modal_open = true;
                    return;
                }
            };

        let sender = self.sender.clone();

        self.runtime.block_on(async {
            tokio::spawn(async move {
                info!("Fetching weather data at ({lat}, {lon})");

                match F::fetch_weather(lat, lon).await {
                    Ok(response) => {
                        if let Err(err) = sender.send(response) {
                            error!("{err}");
                        }
                    }
                    Err(err) => error!("{err}"),
                }
            })
            .await
            .expect("error joining thread");
        });
    }
}

impl<D, F> eframe::App for AppController<D, F>
where
    D: WeatherData + Default + Sync + Send + 'static,
    F: WeatherFetch<Output = D> + Send,
{
    // TODO: re-enable for feature to save state
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     let state = self.state.lock().unwrap();

    //     eframe::set_value(storage, eframe::APP_KEY, &*state);
    // }

    fn update(&mut self, ctx: &Ctx, frame: &mut Frame) {
        if self.fetch_state == FetchState::Requested {
            self.fetch_state = FetchState::InProgress;
            self.fetch();
        }

        if let Ok(true) = self.receiver.has_changed() {
            let new = self.receiver.borrow_and_update();
            self.state.update_data(&*new);
        }

        self.state
            .update(ctx, frame, &self.logs, &mut self.fetch_state);
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
            latitude_str: String::from(A51_LAT),
            longitude_str: String::from(A51_LON),
            weather_view_selected: true,
            widgets: Widgets::new(),
            ..Default::default()
        }
    }

    fn update_location(&mut self, ui: &mut Ui, fetch_state: &mut FetchState) {
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
                ui.add(TextEdit::singleline(&mut self.latitude_str).hint_text(A51_LAT));
                ui.end_row();

                // longitude
                ui.add(Label::new("Longitude: "));
                ui.add(TextEdit::singleline(&mut self.longitude_str).hint_text(A51_LON));

                ui.end_row();
            });

        egui::ScrollArea::vertical().show(ui, |ui| {
            if ui.button("Fetch").clicked() && *fetch_state == FetchState::Completed {
                *fetch_state = FetchState::Requested;
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

    fn update(&mut self, ctx: &Ctx, _frame: &mut Frame, logs: &Logs, fetch_state: &mut FetchState) {
        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .toggle_value(&mut self.weather_view_selected, "Weather")
                    .clicked()
                {
                    self.current_view = View::Weather;
                    self.log_view_selected = false;
                    self.weather_view_selected = true;
                }
                if ui
                    .toggle_value(&mut self.log_view_selected, "Log")
                    .clicked()
                {
                    self.current_view = View::Log;
                    self.weather_view_selected = false;
                    self.log_view_selected = true;
                }
            });
        });

        // Central panel for view content
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                View::Weather => {
                    egui::SidePanel::right("right_panel")
                        .resizable(false)
                        .default_width(200.0)
                        .min_width(200.0)
                        .show(ui.ctx(), |ui| {
                            self.update_location(ui, fetch_state);

                            self.update_widget_toggle_pane(ui);
                        });

                    egui::CentralPanel::default().show(ui.ctx(), |ui| {
                        // widget display area
                        self.widgets.windows(ctx, &mut self.open_widgets);

                        if self.location_error_modal_open {
                            self.show_location_error_modal(ui);
                        }
                    });
                }
                View::Log => {
                    // TODO don't clone the logs each time
                    let logs = logs.get().join("\n");
                    egui::CentralPanel::default().show(ui.ctx(), |ui| {
                        // TODO improvements: highlighting on log syntax, colored differently for log levels
                        let language = "rs";
                        let theme =
                            syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
                        syntax_highlighting::code_view_ui(ui, &theme, &logs, language);
                    });
                }
            }
        });
    }

    fn update_data<D: WeatherData>(&mut self, data: &D) {
        self.widgets.update_data(data);
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
