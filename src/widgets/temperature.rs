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

    fn show(&mut self, ctx: &Context, open: &mut bool, data: &dyn WeatherData) {
        // TODO
    }
}
