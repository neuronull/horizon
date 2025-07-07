#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod widgets;

pub use app::WeatherApp;
pub use widgets::Widgets;

pub const APP_NAME: &str = "horizon";
