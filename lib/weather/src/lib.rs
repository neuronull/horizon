#![warn(clippy::pedantic)]
#![warn(clippy::all, rust_2018_idioms)]

use anyhow::Result;
use async_trait::async_trait;

mod pirate;

pub use pirate::{ForecastResponse as PirateData, PirateWeather};

// At present, just using the pirate data.
// In the future if we support other weather APIs, we'll
// want to introduce a proper abstraction layer.
type DataPoint = pirate::DataPoint;

pub trait WeatherData {
    fn current(&self) -> Option<&DataPoint>;
}

#[async_trait]
pub trait WeatherFetch {
    type Output;

    async fn fetch_weather(lat: f64, lon: f64) -> Result<Self::Output>;
}
