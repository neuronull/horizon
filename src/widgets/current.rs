use egui::{Color32, Context, Ui, Window};
use lib_weather::WeatherData;

use super::{View, Widget};

#[derive(Default)]
pub struct CurrentWidget {}

impl CurrentWidget {}

impl Widget for CurrentWidget {
    fn name(&self) -> &'static str {
        "current"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool, data: &dyn WeatherData) {
        Window::new(self.name())
            .open(open)
            .default_size(egui::vec2(512.0, 256.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui, data));
    }
}

impl View for CurrentWidget {
    fn ui(&mut self, ui: &mut Ui, data: &dyn WeatherData) {
        if let Some(cur) = data.current() {
            if let Some(t) = cur.temperature {
                ui.colored_label(color_of_temp(t), format!("{t}"));
            } else {
                // TODO: choose color
                ui.colored_label(Color32::from_rgb(255, 255, 255), "--");
            }
        }
    }
}

// TODO add logic to compute color from temp ranges
fn color_of_temp(_temp: f64) -> Color32 {
    Color32::from_rgb(128, 140, 255)
}
