use egui::{Context, Ui, Window};

use super::{View, Widget};

#[derive(Default)]
pub struct CurrentWidget {}

impl CurrentWidget {}

impl Widget for CurrentWidget {
    fn name(&self) -> &'static str {
        "current"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new(self.name())
            .open(open)
            .default_size(egui::vec2(512.0, 256.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl View for CurrentWidget {
    fn ui(&mut self, ui: &mut Ui) {
        // TODO
    }
}
