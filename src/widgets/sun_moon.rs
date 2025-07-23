use egui::{Color32, Context, Grid, Ui, Window};
use lib_weather::WeatherData;

use egui::{Painter, Pos2, Rect, Shape, Stroke};
use std::f32::consts::PI;

use super::{View, Widget};

#[derive(Default)]
pub struct SunMoon {
    sunrise: Option<i64>,
    sunset: Option<i64>,
    offset: i64,
    now: i64,
}

impl Widget for SunMoon {
    fn name(&self) -> &'static str {
        "sun & moon"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new(self.name())
            .open(open)
            .default_size(egui::vec2(512.0, 256.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }

    fn update_data(&mut self, data: &dyn WeatherData) {
        let (_, offset) = data.time();
        self.offset = offset as i64;

        if let Some(today) = data.daily().and_then(|days| days.first()) {
            self.sunrise = today.sunrise_time;
            self.sunset = today.sunset_time;
            self.now = today.time;
        }
    }
}

impl View for SunMoon {
    fn ui(&mut self, ui: &mut Ui) {
        if let (Some(sunrise), Some(sunset)) = (self.sunrise, self.sunset) {
            self.draw_sun_arc(ui, sunrise, sunset, self.offset, self.now);
        }
    }
}
impl SunMoon {
    fn draw_sun_arc(&self, ui: &mut Ui, sunrise: i64, sunset: i64, offset: i64, now: i64) {
        let desired_size = egui::vec2(ui.available_width(), 200.0);
        let (response, painter) = ui.allocate_painter(desired_size, egui::Sense::hover());

        let rect = response.rect;
        let center = Pos2::new(rect.center().x, rect.bottom());
        let radius = rect.width().min(rect.height()) / 2.0;

        let timezone_offset_secs = offset * 3600; // adjust to actual timezone

        let seconds_per_day = 24 * 60 * 60;

        let midnight = (sunrise / seconds_per_day) * seconds_per_day; // anchor to midnight

        // Draw dark background behind the arc
        painter.rect_filled(rect, 0.0, Color32::from_rgb(10, 10, 30));

        let segments = 100;
        let t_sunrise =
            ((sunrise + timezone_offset_secs) % seconds_per_day) as f32 / seconds_per_day as f32;
        let t_sunset =
            ((sunset + timezone_offset_secs) % seconds_per_day) as f32 / seconds_per_day as f32;

        for i in 0..segments {
            let t_start = i as f32 / segments as f32;
            let t_end = (i + 1) as f32 / segments as f32;

            let angle_start = PI * t_start;
            let angle_end = PI * t_end;

            let pos_start =
                center + egui::vec2(radius * angle_start.cos(), -radius * angle_start.sin());
            let pos_end = center + egui::vec2(radius * angle_end.cos(), -radius * angle_end.sin());

            let t = i as f32 / 100.0;
            let angle = std::f32::consts::PI * t; // 0 to PI for a half-circle
            let color = sky_color_for_time(t, t_sunrise, t_sunset);

            painter.add(egui::Shape::line_segment(
                [pos_start, pos_end],
                Stroke::new(4.0, color),
            ));
        }

        draw_current_time_dot(&painter, center, radius, now, midnight);

        // Draw ground line
        painter.line_segment(
            [
                Pos2::new(center.x - radius, center.y),
                Pos2::new(center.x + radius, center.y),
            ],
            Stroke::new(2.0, Color32::BLACK),
        );
    }
}

fn sky_color_for_time(time_frac: f32, t_sunrise: f32, t_sunset: f32) -> Color32 {
    if time_frac >= t_sunrise && time_frac <= t_sunset {
        Color32::YELLOW // Daylight
    } else {
        Color32::BLUE // Night
    }
}

fn draw_current_time_dot(painter: &Painter, center: Pos2, radius: f32, now: i64, midnight: i64) {
    let seconds_per_day = 86400.0;
    let t = ((now - midnight) as f32 / seconds_per_day).clamp(0.0, 1.0);
    let angle = PI * (1.0 - t); // same flip

    let dot_pos = center + egui::vec2(radius * angle.cos(), -radius * angle.sin());
    painter.circle_filled(dot_pos, 4.0, Color32::WHITE);
}
