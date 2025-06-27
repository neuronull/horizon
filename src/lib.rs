#![warn(clippy::all, rust_2018_idioms)]

mod app;

pub use app::WeatherApp;

pub const APP_NAME: &str = "horizon";
