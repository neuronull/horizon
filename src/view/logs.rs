use egui::Ui;
use egui_extras::syntax_highlighting;
use tokio::sync::mpsc::{error::TryRecvError, Receiver};
use tracing::error;

pub struct LogsView {
    rx: Receiver<String>,
    logs: Vec<String>,
    logs_display: String,
}

impl LogsView {
    #[must_use]
    pub fn new(rx: Receiver<String>) -> Self {
        Self {
            rx,
            logs: Vec::new(),
            logs_display: String::new(),
        }
    }

    // check for new logs and if found, append them to stored logs
    // the update loop runs with enough frequency and our expected
    // throughput is low enough that checking once each call is more than enough.
    pub fn check_logs(&mut self) {
        match self.rx.try_recv() {
            Ok(log) => {
                self.logs_display.push_str(&log);
                self.logs.push(log);
            }
            Err(TryRecvError::Empty) => {}
            Err(e) => error!("Logs sender disconnected: {e}"),
        }
    }

    pub fn update(&mut self, ui: &Ui) {
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            // TODO improvements: highlighting on log syntax, colored differently for log levels
            let language = "rs";
            let theme = syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
            syntax_highlighting::code_view_ui(ui, &theme, &self.logs_display, language);
        });
    }
}
