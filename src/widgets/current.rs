use egui::{Color32, Context, Grid, Ui, Window};
use lib_weather::WeatherData;

use super::{View, Widget};

#[derive(Default)]
pub struct CurrentWidget {
    // current temp
    current_temp: String,
    current_temp_color: Color32,
    // feels like
    feels_like: String,
    feels_like_color: Color32,
    // precipitation
    precipitation_prob: String,
    // humidity
    humidity: String,
    // pressure
    pressure: String,
    // wind
    wind: String,
    // uv index
    uv_index: String,
    // cloud cover
    cloud_cover: String,
    // visibility
    visibility: String,
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
            self.current_temp = cur
                .temperature
                .map_or("--".to_string(), |v| format!("{}", v.round()));

            self.current_temp_color = if let Some(t) = cur.temperature {
                color_of_temp(t)
            } else {
                Color32::from_rgb(255, 255, 255)
            };

            // feels like
            self.feels_like = cur
                .apparent_temperature
                .map_or("--".to_string(), |v| format!("{}", v.round()));

            self.feels_like_color = if let Some(t) = cur.apparent_temperature {
                color_of_temp(t)
            } else {
                Color32::from_rgb(255, 255, 255)
            };

            self.precipitation_prob = float_to_percent_str(cur.precip_probability);

            self.pressure = float_str(cur.pressure);

            self.humidity = float_to_percent_str(cur.humidity);

            self.wind = float_str(cur.wind_speed);

            self.uv_index = float_str(cur.uv_index);

            self.cloud_cover = float_to_percent_str(cur.cloud_cover);

            self.visibility = float_str(cur.visibility);
        }
    }
}

fn float_to_percent_str(f: Option<f64>) -> String {
    f.map_or("--".to_string(), |v| {
        format!("{}", (v * 100.0).round() as i64)
    })
}

fn float_str(f: Option<f64>) -> String {
    f.map_or("--".to_string(), |v| format!("{v:.1}"))
}

impl View for CurrentWidget {
    fn ui(&mut self, ui: &mut Ui) {
        Grid::new("current_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Current temperature (F): ");
                ui.colored_label(self.current_temp_color, &self.current_temp);
                ui.end_row();

                ui.label("Feels like (F): ");
                ui.colored_label(self.feels_like_color, &self.feels_like);
                ui.end_row();

                ui.label("Percipitation prob (%): ");
                ui.label(&self.precipitation_prob);
                ui.end_row();

                ui.label("Humidity (%): ");
                ui.label(&self.humidity);
                ui.end_row();

                ui.label("Pressure (millibars): ");
                ui.label(&self.pressure);
                ui.end_row();

                ui.label("Wind (mph): ");
                ui.label(&self.wind);
                ui.end_row();

                ui.label("UV Index: ");
                ui.label(&self.uv_index);
                ui.end_row();

                ui.label("Cloud cover (%): ");
                ui.label(&self.cloud_cover);
                ui.end_row();

                ui.label("Visibility (miles): ");
                ui.label(&self.visibility);
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
