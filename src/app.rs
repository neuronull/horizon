use serde::{Deserialize, Serialize};

use eframe::{CreationContext, Frame};
use egui::Context;

#[derive(Deserialize, Serialize)]
pub struct WeatherApp {}

impl Default for WeatherApp {
    fn default() -> Self {
        Self {}
    }
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

    fn update(&mut self, _ctx: &Context, _frame: &mut Frame) {}
}
