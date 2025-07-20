#![warn(clippy::pedantic)]
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod logger;
mod widgets;

pub use app::AppController;
pub use logger::{setup_logging, Logs};
pub use widgets::Widgets;

pub const APP_NAME: &str = "horizon";
