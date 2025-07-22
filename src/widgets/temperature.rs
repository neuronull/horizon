use chrono::DateTime;
use egui::{Color32, Context, Ui, Window};
use egui_plot::{Legend, Line, Plot};
use lib_weather::{DataBlock, WeatherData};

use super::{View, Widget};

#[derive(Default)]
pub struct TemperatureWidget {
    show_daily: bool,
    hourly: Option<DataBlock>,
    daily: Option<DataBlock>,
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
    }
}

impl View for TemperatureWidget {
    fn ui(&mut self, ui: &mut Ui) {
        // ui.horizontal(|ui| {
        // ui.radio_value(&mut self.show_daily, false, "Hourly");
        // ui.radio_value(&mut self.show_daily, true, "Daily");
        // });

        let data = if self.show_daily {
            &self.daily
        } else {
            &self.hourly
        };

        if data.is_none() {
            ui.label("No forecast data available.");
            return;
        }
        let points: Vec<[f64; 2]> = data
            .as_ref()
            .unwrap()
            .iter()
            .map(|dp| [dp.time as f64, dp.temperature.unwrap()])
            .collect();

        let color = if self.show_daily {
            Color32::from_rgb(0, 150, 250) // Blue
        } else {
            Color32::from_rgb(250, 120, 0) // Orange
        };

        let line = Line::new("Temperature (°F)", points)
            .color(color)
            .fill_alpha(0.5);

        Plot::new("temperature_plot")
            .legend(Legend::default())
            .view_aspect(2.5)
            .show_x(true)
            .show_y(true)
            .set_margin_fraction(egui::vec2(0.05, 0.1))
            .x_axis_formatter(move |timestamp, _range| {
                let dt = DateTime::from_timestamp(timestamp.value as i64, 0).unwrap();
                if self.show_daily {
                    dt.format("%a %m/%d").to_string()
                } else {
                    dt.format("%a %H:%M").to_string()
                }
            })
            .label_formatter(|_, val| format!("{:.1}°F", val.y))
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }
}
