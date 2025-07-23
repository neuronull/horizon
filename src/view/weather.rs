use std::collections::BTreeSet;

use egui::{Id, Label, Layout, Modal, ScrollArea, TextEdit, Ui};

use crate::{FetchState, Widgets, A51_LAT, A51_LON};

#[derive(Default)]
pub struct WeatherView {
    /// UI widgets
    pub widgets: Widgets,
    open_widgets: BTreeSet<String>,
    /// location latitude
    pub latitude_str: String,
    /// location longitude
    pub longitude_str: String,
    pub location_error_modal_open: bool,
}

impl WeatherView {
    #[must_use]
    pub fn new() -> Self {
        Self {
            latitude_str: String::from(A51_LAT),
            longitude_str: String::from(A51_LON),
            widgets: Widgets::new(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, ui: &mut Ui, fetch_state: &mut FetchState) {
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .show(ui.ctx(), |ui| {
                self.update_location(ui, fetch_state);

                self.update_widget_toggle_pane(ui);
            });

        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            // widget display area
            self.widgets.windows(ui.ctx(), &mut self.open_widgets);

            if self.location_error_modal_open {
                self.show_location_error_modal(ui);
            }
        });
    }

    fn update_location(&mut self, ui: &mut Ui, fetch_state: &mut FetchState) {
        ui.vertical_centered(|ui| {
            ui.heading("Location");
        });
        ui.separator();

        egui::Grid::new("location_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.add(Label::new("Latitude: "));

                // latitude
                ui.add(TextEdit::singleline(&mut self.latitude_str).hint_text(A51_LAT));
                ui.end_row();

                // longitude
                ui.add(Label::new("Longitude: "));
                ui.add(TextEdit::singleline(&mut self.longitude_str).hint_text(A51_LON));

                ui.end_row();
            });

        egui::ScrollArea::vertical().show(ui, |ui| {
            if ui.button("Fetch").clicked() && *fetch_state == FetchState::Completed {
                *fetch_state = FetchState::Requested;
            }
        });
        ui.separator();
    }

    fn show_location_error_modal(&mut self, ui: &mut Ui) {
        Modal::new(Id::new("location_error_modal")).show(ui.ctx(), |ui| {
            ui.set_width(200.0);
            ui.heading("Location invalid.");

            ui.add_space(32.0);

            egui::Sides::new().show(
                ui,
                |_ui| {},
                |ui| {
                    if ui.button("close").clicked() {
                        self.location_error_modal_open = false;
                    }
                },
            );
        });
    }

    // display widget selectors
    fn update_widget_toggle_pane(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Widgets");

            ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    self.widgets.checkboxes(ui, &mut self.open_widgets);
                });
            });
        });
        ui.separator();
    }
}
