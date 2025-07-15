use egui::Context;

use super::Widget;

#[derive(Default)]
pub struct CurrentWidget {}

impl CurrentWidget {}

impl Widget for CurrentWidget {
    fn name(&self) -> &'static str {
        "current"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        // TODO
    }
}
