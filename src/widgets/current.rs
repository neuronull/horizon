use egui::{Color32, Context, Grid, Ui, Window};
use lib_weather::WeatherData;

use super::{View, Widget};

#[derive(Default)]
pub struct CurrentWidget {
    // current temp
    current_temp: Option<i16>,
    current_temp_color: Color32,
    // feels like
    feels_like: Option<i16>,
    feels_like_color: Color32,
}

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

    fn update_data(&mut self, data: &dyn WeatherData) {
        if let Some(cur) = data.current() {
            // current temp
            self.current_temp = cur.temperature.map(|t| t.round() as i16);
            if let Some(t) = cur.temperature {
                self.current_temp_color = color_of_temp(t);
            } else {
                // TODO: choose color
                self.current_temp_color = Color32::from_rgb(255, 255, 255);
            }

            // feels like
            self.feels_like = cur.apparent_temperature.map(|t| t.round() as i16);
            if let Some(t) = cur.apparent_temperature {
                self.feels_like_color = color_of_temp(t);
            } else {
                // TODO: choose color
                self.feels_like_color = Color32::from_rgb(255, 255, 255);
            }
        }
    }
}

impl View for CurrentWidget {
    fn ui(&mut self, ui: &mut Ui) {
        Grid::new("current_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                // current temp
                ui.label("Current temperature (F): ");
                if let Some(t) = self.current_temp {
                    ui.colored_label(self.current_temp_color, format!("{t}"));
                } else {
                    ui.colored_label(self.current_temp_color, "--");
                }
                ui.end_row();

                // feels like
                ui.label("Feels like (F): ");
                if let Some(t) = self.feels_like {
                    ui.colored_label(self.feels_like_color, format!("{t}"));
                } else {
                    ui.colored_label(self.feels_like_color, "--");
                }
                ui.end_row();
            });
    }
}

// compute color from temp ranges
fn color_of_temp(temp: f64) -> Color32 {
    // Clamp the temperature between -20°F and 120°F
    let clamped = temp.clamp(-20.0, 120.0);

    // Normalize to 0.0 - 1.0
    let t = (clamped + 20.0) / 140.0;

    // Interpolate between blue (cold) → cyan → green → yellow → red (hot)
    let r = (255.0 * t.powf(1.5)).min(255.0) as u8;
    let g = (255.0 * (1.0 - (2.0 * (t - 0.5)).abs())).min(255.0) as u8;
    let b = (255.0 * (1.0 - t).powf(2.0)).min(255.0) as u8;

    Color32::from_rgb(r, g, b)
}
