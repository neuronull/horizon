use egui::Context;

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
}
