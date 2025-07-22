use egui::Ui;
use egui_extras::syntax_highlighting;

use crate::{setup_logging, Logs};

#[derive(Default)]
pub struct LogsView {
    logger: Logs,
}

impl LogsView {
    #[must_use]
    pub fn new() -> Self {
        let logger = setup_logging();

        Self { logger }
    }

    pub fn update(&mut self, ui: &Ui) {
        // TODO dont clone each update
        let logs = self.logger.get().join("\n");

        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            // TODO improvements: highlighting on log syntax, colored differently for log levels
            let language = "rs";
            let theme = syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
            syntax_highlighting::code_view_ui(ui, &theme, &logs, language);
        });
    }
}
