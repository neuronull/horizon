use std::collections::BTreeSet;

use anyhow::Result;
use egui::{Id, IntoAtoms, Label, Layout, Modal, ScrollArea, TextEdit, Ui};
use tokio::{
    runtime::Handle,
    sync::watch::{self, Receiver, Sender},
};
use tracing::{error, info};

use crate::{FetchState, Widgets, A51_LAT, A51_LON};
use lib_geolocate::{get_geo_location, GeoResponse};

pub struct WeatherView {
    rt: Option<Handle>,
    /// UI widgets
    pub widgets: Widgets,
    open_widgets: BTreeSet<String>,
    /// location latitude
    pub latitude_str: String,
    /// location longitude
    pub longitude_str: String,
    pub location_error_modal_open: bool,
    tooltips_enabled: bool,
    sender: Sender<Result<GeoResponse>>,
    receiver: Receiver<Result<GeoResponse>>,
}

impl WeatherView {
    #[must_use]
    pub fn new(rt: Option<Handle>) -> Self {
        let (sender, receiver) = watch::channel(Ok(GeoResponse::default()));
        Self {
            rt,
            widgets: Widgets::new(),
            open_widgets: BTreeSet::new(),
            latitude_str: String::from(A51_LAT),
            longitude_str: String::from(A51_LON),
            location_error_modal_open: false,
            tooltips_enabled: false,
            sender,
            receiver,
        }
    }

    pub fn update(&mut self, ui: &mut Ui, fetch_state: &mut FetchState) {
        if let Ok(true) = self.receiver.has_changed() {
            match &*self.receiver.borrow_and_update() {
                Ok(geo) => {
                    self.latitude_str = geo.location.latitude.clone();
                    self.longitude_str = geo.location.longitude.clone();
                }
                Err(err) => {
                    error!("{err}");
                }
            }
        }

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

                let mut geobutton = ui.button("Geolocate");
                if self.tooltips_enabled {
                    geobutton = geobutton.on_hover_ui(|ui| {
                        ui.label("Click to resolve latitude and longitude from your current location.");
                    });
                }
                if geobutton.clicked() {
                    self.request_locate();
                }

                let mut fetchbutton = ui.button("Fetch");
                if self.tooltips_enabled {
                    fetchbutton = fetchbutton.on_hover_ui(|ui| {
                        ui.label("Click to obtain weather data from the selected latitude and longitude.");
                    });
                }
                if fetchbutton.clicked() && *fetch_state == FetchState::Completed {
                    *fetch_state = FetchState::Requested;
                }

                ui.end_row();
            });

        ui.separator();
    }

    fn request_locate(&mut self) {
        info!("Geolocating...");
        let sender = self.sender.clone();

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                wasm_bindgen_futures::spawn_local(async move {
                    let result = get_geo_location().await;
                    if let Err(err) = sender.send(result) {
                        error!("{err}");
                    } else {
                        info!("Geolocate success.");
                    }
                });
            } else {
                let result = self.rt.as_ref().unwrap().block_on(get_geo_location());
                if let Err(err) = sender.send(result) {
                    error!("{err}");
                } else {
                    info!("Geolocate success.");
                }
            }
        }
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
                    self.widgets
                        .checkboxes(ui, &mut self.open_widgets, self.tooltips_enabled);
                });
            });
        });
        ui.separator();

        let mut organize = ui.button("Organize widgets");
        if self.tooltips_enabled {
            organize = organize.on_hover_ui(|ui| {
                ui.label("Click to automatically neatly arrange open widgets.");
            });
        }

        if organize.clicked() {
            ui.ctx().memory_mut(egui::Memory::reset_areas);
        }
        ui.toggle_value(
            &mut self.tooltips_enabled,
            ("Tooltips enabled").into_atoms(),
        )
        .on_hover_ui(|ui| {
            ui.label(format!(
                "Click to {} tooltips.",
                if self.tooltips_enabled {
                    "disable"
                } else {
                    "enable"
                }
            ));
        });
    }
}
