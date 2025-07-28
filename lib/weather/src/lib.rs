#![warn(clippy::pedantic)]
#![warn(clippy::all, rust_2018_idioms)]

use anyhow::Result;
use async_trait::async_trait;

mod pirate;

pub use pirate::{ForecastResponse as PirateData, PirateWeather};

// At present, just using the pirate data.
// In the future if we support other weather APIs, we'll
// want to introduce a proper abstraction layer.
pub type DataPoint = pirate::DataPoint;
pub type DataBlock = Vec<DataPoint>;

pub trait WeatherData {
    fn current(&self) -> Option<&DataPoint>;

    fn hourly(&self) -> Option<&DataBlock>;

    fn daily(&self) -> Option<&DataBlock>;

    fn time(&self) -> (&str, f64);
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait WeatherFetch {
    type Output;

    async fn fetch_weather(lat: f64, lon: f64) -> Result<Self::Output>;
}
