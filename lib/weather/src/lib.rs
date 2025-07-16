#![warn(clippy::pedantic)]
#![warn(clippy::all, rust_2018_idioms)]

use anyhow::Result;
use async_trait::async_trait;

mod pirate;

pub use pirate::{ForecastResponse as PirateData, PirateWeather};

pub trait WeatherData {
    // TODO we'll create an abstraction layer later
    fn current(&self) -> Option<&pirate::DataPoint>;
}

#[async_trait]
pub trait WeatherFetch {
    type Output;

    async fn fetch_weather(lat: f64, lon: f64) -> Result<Self::Output>;
}
