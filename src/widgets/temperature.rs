use egui::{Context, Ui, Window};
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
            .enumerate()
            .map(|(i, point)| {
                // Convert timestamp to hour string
                let time_label = time::OffsetDateTime::from_unix_timestamp(point.time)
                    .ok()
                    .and_then(|dt| {
                        dt.format(&time::format_description::parse("[hour]:[minute]").ok()?)
                            .ok()
                    })
                    .unwrap_or_else(|| i.to_string());

                egui_plot::Bar::new(i as f64, point.temperature.unwrap())
                    .width(0.8)
                    .name(time_label)
                    .fill(temperature_color(point.temperature.unwrap()))
            })
            .collect();

        let chart = egui_plot::BarChart::new("Temperature", bars);

        egui_plot::Plot::new("temp_bar_chart")
            .height(200.0)
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
