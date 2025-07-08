use async_trait::async_trait;
use std::error::Error;

mod pirate;

pub use pirate::{ForecastResponse as PirateData, PirateWeather};

pub trait WeatherData {
    fn new() -> Self;
}

#[async_trait]
pub trait WeatherFetch {
    type Output;

    async fn fetch_weather(lat: f64, lon: f64) -> Result<Self::Output, Box<dyn Error>>;
}
