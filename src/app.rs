use anyhow::{Context, Result};
use eframe::Frame;
use egui::Context as Ctx;
use std::marker::PhantomData;

#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::{Builder, Runtime};

use tokio::sync::mpsc;
use tokio::sync::watch::{Receiver, Sender};
use tracing::{error, info};

use super::{LogsView, WeatherView};
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
    D: WeatherData + Default + Sync + 'static,
    F: WeatherFetch<Output = D>,
{
    sender: Sender<Result<D>>,
    receiver: Receiver<Result<D>>,
    #[cfg(not(target_arch = "wasm32"))]
    runtime: Runtime,
    state: AppState,
    /// Weather data
    pub data: D,
    _fetcher: PhantomData<F>,
}

impl<D, F> AppController<D, F>
where
    D: WeatherData + Default + Sync + 'static,
    F: WeatherFetch<Output = D>,
{
    /// # Panics
    ///
    /// Will panic if tokio runtime build fails
    #[must_use]
    pub fn new(state: AppState) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build runtime");

        let data = D::default();
        let (sender, receiver) = tokio::sync::watch::channel(Ok(D::default()));

        info!("Initializing app");

        let mut controller = Self {
            sender,
            receiver,
            #[cfg(not(target_arch = "wasm32"))]
            runtime,
            state,
            data,
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
                self.state.fetch_state = FetchState::Completed;
                return;
            }
        };

        let sender = self.sender.clone();

        info!("Fetching weather data at ({lat}, {lon})");

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = F::fetch_weather(lat, lon).await;
                    if let Err(err) = sender.send(response) {
                        error!("{err}");
                    }
                });
            } else {
                self.runtime.block_on(async {
                    let response = F::fetch_weather(lat, lon).await;
                    if let Err(err) = sender.send(response) {
                        error!("{err}");
                    }
                });
            }
        }
    }

    fn update(&mut self, ctx: &Ctx) {
        if self.state.fetch_state == FetchState::Requested {
            self.state.fetch_state = FetchState::InProgress;
            self.fetch();
        }

        if let Ok(true) = self.receiver.has_changed() {
            match &*self.receiver.borrow_and_update() {
                Ok(data) => {
                    self.state.update_data(data);
                }
                Err(err) => {
                    error!("{err}");
                }
            }
            self.state.fetch_state = FetchState::Completed;
        }

        self.state.update(ctx);
    }
}

impl<D, F> eframe::App for AppController<D, F>
where
    D: WeatherData + Default + Sync + 'static,
    F: WeatherFetch<Output = D>,
{
    // TODO: re-enable for feature to save state
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     let state = self.state.lock().unwrap();

    //     eframe::set_value(storage, eframe::APP_KEY, &*state);
    // }

    fn update(&mut self, ctx: &Ctx, _frame: &mut Frame) {
        self.update(ctx);
    }
}

#[derive(PartialEq, Default)]
enum View {
    #[default]
    Weather,
    Log,
}

pub struct AppState {
    // UI view state
    active_view: View,
    log_view_selected: bool,
    weather_view_selected: bool,
    pub weather_view: WeatherView,
    pub logs_view: LogsView,
    /// state of the weather data fetch operation
    fetch_state: FetchState,
}

impl AppState {
    /// Called once before the first frame.
    pub fn new(logrx: mpsc::Receiver<String>) -> Self {
        // TODO: re-enable for feature to save state
        // Load previous app state (if any).
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Self {
            weather_view_selected: true,
            weather_view: WeatherView::new(),
            logs_view: LogsView::new(logrx),
            active_view: View::default(),
            log_view_selected: false,
            fetch_state: FetchState::default(),
        }
    }

    fn update(&mut self, ctx: &Ctx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Top menu bar
            egui::TopBottomPanel::top("menu_bar").show(ui.ctx(), |ui| {
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
            egui::CentralPanel::default().show(ui.ctx(), |ui| match self.active_view {
                View::Weather => {
                    self.weather_view.update(ui, &mut self.fetch_state);
                }
                View::Log => {
                    self.logs_view.update(ui);
                }
            });
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
            .trim()
            .replace("°", "")
            .parse::<f64>()
            .context("Invalid input: number not parseable as float.")
    };
    Ok((validate(lat)?, validate(lon)?))
}

#[cfg(test)]
mod test {
    use anyhow::anyhow;
    use egui_kittest::kittest::Queryable;
    use egui_kittest::Harness;
    use lib_weather::PirateData;

    use super::*;

    #[test]
    fn validate_lat_lon_happy() {
        assert_eq!(
            (37.233, -115.800),
            validate_lat_lon_input(crate::A51_LAT, crate::A51_LON).unwrap()
        );
        assert_eq!(
            (37.0, -115.0),
            validate_lat_lon_input("37°", "-115°").unwrap()
        );
    }

    #[test]
    fn validate_lat_lon_happy_with_degrees() {
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

    struct StubWeatherFails {}

    #[async_trait::async_trait]
    impl WeatherFetch for StubWeatherFails {
        type Output = PirateData;

        async fn fetch_weather(_lat: f64, _lon: f64) -> Result<Self::Output> {
            Err(anyhow!("Failed to fetch weather"))
        }
    }

    struct StubWeatherSucceeds {}

    #[async_trait::async_trait]
    impl WeatherFetch for StubWeatherSucceeds {
        type Output = PirateData;

        async fn fetch_weather(_lat: f64, _lon: f64) -> Result<Self::Output> {
            Ok(PirateData::default())
        }
    }

    #[test]
    fn fetch_failure_marked_as_completed() {
        let (_logtx, logrx) = mpsc::channel::<String>(100);
        let state = AppState::new(logrx);

        let initial_state = AppController::<PirateData, StubWeatherFails>::new(state);

        let mut harness = Harness::new_state(
            |ctx, initial_state| {
                initial_state.update(ctx);
            },
            initial_state,
        );

        harness.get_by_label("Latitude: ").type_text("1");
        harness.get_by_label("Latitude: ").type_text("2");
        harness.get_by_label("Fetch").click();

        harness.run();

        assert!(harness.state().state.fetch_state == FetchState::Completed);
    }

    #[test]
    fn fetch_success_marked_as_completed() {
        let (_logtx, logrx) = mpsc::channel::<String>(100);
        let state = AppState::new(logrx);
        let initial_state = AppController::<PirateData, StubWeatherSucceeds>::new(state);

        let mut harness = Harness::new_state(
            |ctx, initial_state| {
                initial_state.update(ctx);
            },
            initial_state,
        );

        harness.get_by_label("Latitude: ").type_text("1");
        harness.get_by_label("Latitude: ").type_text("2");
        harness.get_by_label("Fetch").click();

        harness.run();

        assert!(harness.state().state.fetch_state == FetchState::Completed);
    }
}
