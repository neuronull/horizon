use std::collections::BTreeSet;

use egui::{Context, Ui};
use lib_weather::WeatherData;

mod current;
mod sun_moon;
mod temperature;

pub trait View {
    fn ui(&mut self, ui: &mut Ui);
}

trait Widget {
    /// Is the widget enabled
    fn is_enabled(&self, _ctx: &Context) -> bool {
        true
    }

    /// Name of the widget
    fn name(&self) -> &'static str;

    /// Render the widget visible
    fn show(&mut self, ctx: &Context, open: &mut bool);

    /// Take what the widget needs from the latest `WeatherData`,
    /// run any transformations once, and cache it in the widget's state.
    fn update_data(&mut self, data: &dyn WeatherData);
}

#[derive(Default)]
pub struct Widgets {
    widgets: Vec<Box<dyn Widget + Send>>,
}

impl Widgets {
    #[must_use]
    pub fn new() -> Self {
        Self {
            widgets: vec![
                Box::<temperature::TemperatureWidget>::default(),
                Box::<current::CurrentWidget>::default(),
                Box::<sun_moon::SunMoon>::default(),
            ],
        }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui, open: &mut BTreeSet<String>) {
        for widget in self.widgets.as_mut_slice() {
            if widget.is_enabled(ui.ctx()) {
                let mut is_open = open.contains(widget.name());
                ui.toggle_value(&mut is_open, widget.name());
                set_open(open, widget.name(), is_open);
            }
        }
    }

    pub fn windows(&mut self, ctx: &Context, open: &mut BTreeSet<String>) {
        for widget in self.widgets.as_mut_slice() {
            let mut is_open = open.contains(widget.name());
            widget.show(ctx, &mut is_open);
            set_open(open, widget.name(), is_open);
        }
    }

    pub fn update_data(&mut self, data: &impl WeatherData) {
        for widget in self.widgets.as_mut_slice() {
            widget.update_data(data);
        }
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}
