#![warn(clippy::pedantic)]
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod logger;
mod view;
mod widgets;

pub use app::{AppController, FetchState};
pub use logger::{setup_logging, Logs};
pub use view::{LogsView, WeatherView};
pub use widgets::Widgets;

pub const APP_NAME: &str = "horizon";
pub const A51_LAT: &str = "37.233";
pub const A51_LON: &str = "-115.800";
