#![warn(clippy::pedantic)]
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod widgets;

pub use app::AppController;
pub use widgets::Widgets;

pub const APP_NAME: &str = "horizon";
