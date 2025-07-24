use std::sync::{Arc, Mutex};

use egui::Ui;
use egui_extras::syntax_highlighting;

#[derive(Default)]
pub struct LogsView {
    logs: Arc<Mutex<Vec<String>>>,
}

impl LogsView {
    #[must_use]
    pub fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self { logs }
    }

    /// # Panics
    ///
    /// Will panic if failure to acquire lock
    pub fn update(&mut self, ui: &Ui) {
        let logs = self.logs.lock().unwrap().join("\n");

        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            // TODO improvements: highlighting on log syntax, colored differently for log levels
            let language = "rs";
            let theme = syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
            syntax_highlighting::code_view_ui(ui, &theme, &logs, language);
        });
    }
}
