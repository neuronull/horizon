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
    // precipitation
    precipitation_prob: Option<i64>,
    // humidity
    humidity: Option<i64>,
    // pressure
    pressure: Option<f64>,
    // wind
    wind: Option<f64>,
    // uv index
    uv_index: Option<f64>,
    // cloud cover
    cloud_cover: Option<i64>,
    // visibility
    visibility: Option<f64>,
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
                self.current_temp_color = Color32::from_rgb(255, 255, 255);
            }

            // feels like
            self.feels_like = cur.apparent_temperature.map(|t| t.round() as i16);
            if let Some(t) = cur.apparent_temperature {
                self.feels_like_color = color_of_temp(t);
            } else {
                self.feels_like_color = Color32::from_rgb(255, 255, 255);
            }

            self.precipitation_prob = cur.precip_probability.map(|p| (p * 100.0).round() as i64);
            dbg!(cur.precip_probability);

            self.pressure = cur.pressure;

            self.humidity = cur.humidity.map(|h| (h * 100.0).round() as i64);
            dbg!(cur.humidity);

            self.wind = cur.wind_speed;

            self.uv_index = cur.uv_index;

            self.cloud_cover = cur.cloud_cover.map(|c| (c * 100.0).round() as i64);
            dbg!(cur.cloud_cover);

            self.visibility = cur.visibility;
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

                // precipitation
                ui.label("Percipitation prob (%): ");
                ui.label(
                    self.precipitation_prob
                        .map_or("--".to_string(), |v| format!("{v}"))
                        .to_string(),
                );
                ui.end_row();

                // humidity
                ui.label("Humidity (%): ");
                ui.label(
                    self.humidity
                        .map_or("--".to_string(), |v| format!("{v}"))
                        .to_string(),
                );
                ui.end_row();

                ui.label("Pressure (millibars): ");
                ui.label(
                    self.pressure
                        .map_or("--".to_string(), |v| format!("{v:.1}"))
                        .to_string(),
                );
                ui.end_row();

                // wind (mph)
                ui.label("Wind (mph): ");
                ui.label(
                    self.wind
                        .map_or("--".to_string(), |v| format!("{v:.1}"))
                        .to_string(),
                );
                ui.end_row();

                // uv index ()
                ui.label("UV Index: ");
                ui.label(
                    self.uv_index
                        .map_or("--".to_string(), |v| format!("{v:.1}"))
                        .to_string(),
                );
                ui.end_row();

                // cloud cover (%)
                ui.label("Cloud cover (%): ");
                ui.label(
                    self.cloud_cover
                        .map_or("--".to_string(), |v| format!("{v}"))
                        .to_string(),
                );
                ui.end_row();

                // visibility (miles)
                ui.label("Visibility (miles): ");
                ui.label(
                    self.visibility
                        .map_or("--".to_string(), |v| format!("{v:.1}"))
                        .to_string(),
                );
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
