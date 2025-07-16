use std::collections::BTreeSet;

use egui::{Context, Ui};

mod current;
mod temperature;

trait Widget {
    /// Is the widget enabled
    fn is_enabled(&self, _ctx: &Context) -> bool {
        true
    }

    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &Context, open: &mut bool);
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
            ],
        }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui, open: &mut BTreeSet<String>) {
        let Self { widgets } = self;
        for widget in widgets {
            if widget.is_enabled(ui.ctx()) {
                let mut is_open = open.contains(widget.name());
                ui.toggle_value(&mut is_open, widget.name());
                set_open(open, widget.name(), is_open);
            }
        }
    }

    pub fn windows(&mut self, ctx: &Context, open: &mut BTreeSet<String>) {
        let Self { widgets } = self;
        for widget in widgets {
            let mut is_open = open.contains(widget.name());
            widget.show(ctx, &mut is_open);
            set_open(open, widget.name(), is_open);
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
