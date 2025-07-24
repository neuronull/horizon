use chrono::DateTime;
use chrono_tz::Tz;
use egui::{Context, Ui, Window};
use egui_plot::{Corner, Legend, Plot};
use lib_weather::{DataBlock, WeatherData};

use super::{View, Widget};

#[derive(Default)]
pub struct TemperatureWidget {
    show_daily: bool,
    hourly: Option<DataBlock>,
    daily: Option<DataBlock>,
    timezone: String,
}

impl Widget for TemperatureWidget {
    fn name(&self) -> &'static str {
        "temperature"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new(self.name())
            .open(open)
            .default_size(egui::vec2(512.0, 256.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }

    fn update_data(&mut self, data: &dyn WeatherData) {
        self.hourly = data.hourly().cloned();
        self.daily = data.daily().cloned();
        let time = data.time();
        time.0.clone_into(&mut self.timezone);
    }
}

impl View for TemperatureWidget {
    fn ui(&mut self, ui: &mut Ui) {
        // ui.horizontal(|ui| {
        // ui.radio_value(&mut self.show_daily, false, "Hourly");
        // ui.radio_value(&mut self.show_daily, true, "Daily");
        // });
        //

        let data = if self.show_daily {
            &self.daily
        } else {
            &self.hourly
        };

        if let Some(data) = data {
            self.temp_bar_chart(ui, data);
        } else {
            ui.label("No forecast data available.");
        }
    }
}

impl TemperatureWidget {
    fn temp_bar_chart(&self, ui: &mut Ui, data: &DataBlock) {
        let bars: Vec<egui_plot::Bar> = data
            .iter()
            .map(|point| {
                let x = point.time as f64;
                let y = point.temperature.unwrap_or(0.0);
                egui_plot::Bar::new(x, y)
                    .width(300.0) // about 5 minutes wide
                    .fill(temperature_color(y))
            })
            .collect();

        let chart = egui_plot::BarChart::new("Temperature", bars);
        let tz: Tz = self.timezone.parse().unwrap_or(chrono_tz::UTC);

        Plot::new("temp_bar_chart")
            .legend(Legend::default().position(Corner::RightTop))
            .x_axis_formatter({
                move |x, _range| {
                    DateTime::from_timestamp(x.value as i64, 0)
                        .map(|dt| dt.with_timezone(&tz))
                        .map_or_else(|| "--:--".into(), |dt| dt.format("%H:%M").to_string())
                }
            })
            .y_axis_formatter(|y, _| format!("{:.0}°F", y.value))
            .label_formatter({
                move |name, value| {
                    let time_str = DateTime::from_timestamp(value.x as i64, 0)
                        .map(|dt| dt.with_timezone(&tz))
                        .map_or_else(|| "--:--".into(), |dt| dt.format("%H:%M").to_string());

                    format!("{name}: {:.1}°F at {}", value.y, time_str)
                }
            })
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(chart);
            });
    }
}

fn temperature_color(temp: f64) -> egui::Color32 {
    match temp {
        t if t < 32.0 => egui::Color32::from_rgb(135, 206, 250), // freezing — light blue
        t if t < 50.0 => egui::Color32::from_rgb(173, 216, 230), // cold — pale blue
        t if t < 65.0 => egui::Color32::from_rgb(255, 255, 153), // cool — light yellow
        t if t < 75.0 => egui::Color32::from_rgb(255, 215, 0),   // pleasant — gold
        t if t < 85.0 => egui::Color32::from_rgb(255, 165, 0),   // warm — orange
        t if t < 95.0 => egui::Color32::from_rgb(255, 99, 71),   // hot — tomato
        _ => egui::Color32::from_rgb(178, 34, 34),               // very hot — firebrick
    }
}
