use std::marker::{PhantomData, Send};
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use eframe::{CreationContext, Frame};
use egui::Context as Ctx;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::watch::{Receiver, Sender};
use tracing::{error, info};

use super::{setup_logging, LogsView, WeatherView};
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

        let logs = Arc::new(Mutex::new(Vec::<String>::new()));
        setup_logging(Arc::clone(&logs));

        let data = D::default();
        let state = AppState::new(cc, logs);
        let (sender, receiver) = tokio::sync::watch::channel(D::default());

        info!("Initializing app");

        let mut controller = Self {
            sender,
            receiver,
            runtime,
            state,
            data,
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
        let (lat, lon) = match validate_lat_lon_input(
            &self.state.weather_view.latitude_str,
            &self.state.weather_view.longitude_str,
        ) {
            Ok(res) => res,
            Err(err) => {
                error!("Invalid location submitted: {err}");
                self.state.weather_view.location_error_modal_open = true;
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

        self.state.update(ctx, frame, &mut self.fetch_state);
    }
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
    active_view: View,
    log_view_selected: bool,
    weather_view_selected: bool,
    pub weather_view: WeatherView,
    pub logs_view: LogsView,
}

impl AppState {
    /// Called once before the first frame.
    pub fn new(_cc: &CreationContext<'_>, logs: Arc<Mutex<Vec<String>>>) -> Self {
        // TODO: re-enable for feature to save state
        // Load previous app state (if any).
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Self {
            weather_view_selected: true,
            weather_view: WeatherView::new(),
            logs_view: LogsView::new(logs),
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Ctx, _frame: &mut Frame, fetch_state: &mut FetchState) {
        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // hopefully there is a cleaner way to acheive this:
                if ui
                    .toggle_value(&mut self.weather_view_selected, "Weather")
                    .clicked()
                {
                    self.active_view = View::Weather;
                    self.log_view_selected = false;
                    self.weather_view_selected = true;
                }
                if ui
                    .toggle_value(&mut self.log_view_selected, "Log")
                    .clicked()
                {
                    self.active_view = View::Log;
                    self.weather_view_selected = false;
                    self.log_view_selected = true;
                }
            });
        });

        // Central panel for view content
        egui::CentralPanel::default().show(ctx, |ui| match self.active_view {
            View::Weather => {
                self.weather_view.update(ui, fetch_state);
            }
            View::Log => {
                self.logs_view.update(ui);
            }
        });
    }

    fn update_data<D: WeatherData>(&mut self, data: &D) {
        self.weather_view.widgets.update_data(data);
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
            validate_lat_lon_input(crate::A51_LAT, crate::A51_LON).unwrap()
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
