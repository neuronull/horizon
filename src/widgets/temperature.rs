use egui::Context;
use lib_weather::WeatherData;

use super::Widget;

#[derive(Default)]
pub struct TemperatureWidget {}

impl TemperatureWidget {}

impl Widget for TemperatureWidget {
    fn name(&self) -> &'static str {
        "temperature"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        // TODO
    }
    fn update_data(&mut self, data: &dyn WeatherData) {
        // TODO
    }
}
