use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use super::{WeatherData, WeatherFetch};

const BASE_URL: &str = "https://api.pirateweather.net";

pub struct PirateWeather {}

#[async_trait]
impl WeatherFetch for PirateWeather {
    type Output = ForecastResponse;

    async fn fetch_weather(lat: f64, lon: f64) -> Result<Self::Output> {
        let api_key = std::env::var("PIRATEWEATHER_API_KEY")?;
        let url = format!("{BASE_URL}/forecast/{api_key}/{lat},{lon}?units=us");

        let client = Client::new();
        let forecast = client
            .get(&url)
            .send()
            .await?
            .json::<ForecastResponse>()
            .await?;

        println!("aye... success pirate forecast");

        Ok(forecast)
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct ForecastResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub offset: f64,
    pub elevation: Option<f64>,
    pub currently: Option<DataPoint>,
    pub minutely: Option<DataBlock>,
    pub hourly: Option<DataBlock>,
    pub daily: Option<DataBlock>,
    pub alerts: Option<Vec<Alert>>,
    pub flags: Flags,
}

impl WeatherData for ForecastResponse {
    fn current(&self) -> Option<&DataPoint> {
        self.currently.as_ref()
    }
}

#[derive(Debug, Deserialize)]
pub struct DataBlock {
    pub summary: String,
    pub icon: String,
    pub data: Vec<DataPoint>,
}

#[derive(Debug, Deserialize)]
pub struct DataPoint {
    pub time: i64,
    pub summary: Option<String>,
    pub icon: Option<String>,

    #[serde(rename = "precipIntensity")]
    pub precip_intensity: Option<f64>,
    #[serde(rename = "precipIntensityError")]
    pub precip_intensity_error: Option<f64>,
    #[serde(rename = "precipProbability")]
    pub precip_probability: Option<f64>,
    #[serde(rename = "precipType")]
    pub precip_type: Option<String>,
    #[serde(rename = "precipAccumulation")]
    pub precip_accumulation: Option<f64>,

    pub temperature: Option<f64>,
    #[serde(rename = "apparentTemperature")]
    pub apparent_temperature: Option<f64>,

    #[serde(rename = "dewPoint")]
    pub dew_point: Option<f64>,
    pub humidity: Option<f64>,
    pub pressure: Option<f64>,

    #[serde(rename = "windSpeed")]
    pub wind_speed: Option<f64>,
    #[serde(rename = "windGust")]
    pub wind_gust: Option<f64>,
    #[serde(rename = "windBearing")]
    pub wind_bearing: Option<f64>,

    #[serde(rename = "cloudCover")]
    pub cloud_cover: Option<f64>,
    #[serde(rename = "uvIndex")]
    pub uv_index: Option<f64>,
    pub visibility: Option<f64>,
    pub ozone: Option<f64>,

    // v2 extras
    #[serde(rename = "nearestStormDistance")]
    pub nearest_storm_distance: Option<f64>,
    #[serde(rename = "nearestStormBearing")]
    pub nearest_storm_bearing: Option<f64>,
    pub smoke: Option<f64>,
    #[serde(rename = "iceAccumulation")]
    pub ice_accumulation: Option<f64>,
    #[serde(rename = "liquidAccumulation")]
    pub liquid_accumulation: Option<f64>,
    #[serde(rename = "snowAccumulation")]
    pub snow_accumulation: Option<f64>,
    #[serde(rename = "fireIndex")]
    pub fire_index: Option<f64>,
    #[serde(rename = "fireIndexMax")]
    pub fire_index_max: Option<f64>,
    #[serde(rename = "fireIndexMaxTime")]
    pub fire_index_max_time: Option<i64>,
    #[serde(rename = "smokeMax")]
    pub smoke_max: Option<f64>,
    #[serde(rename = "smokeMaxTime")]
    pub smoke_max_time: Option<i64>,

    // daily
    #[serde(rename = "sunriseTime")]
    pub sunrise_time: Option<i64>,
    #[serde(rename = "sunsetTime")]
    pub sunset_time: Option<i64>,
    #[serde(rename = "moonPhase")]
    pub moon_phase: Option<f64>,
    #[serde(rename = "temperatureHigh")]
    pub temperature_high: Option<f64>,
    #[serde(rename = "temperatureHighTime")]
    pub temperature_high_time: Option<i64>,
    #[serde(rename = "temperatureLow")]
    pub temperature_low: Option<f64>,
    #[serde(rename = "temperatureLowTime")]
    pub temperature_low_time: Option<i64>,
    #[serde(rename = "temperatureMin")]
    pub temperature_min: Option<f64>,
    #[serde(rename = "temperatureMinTime")]
    pub temperature_min_time: Option<i64>,
    #[serde(rename = "temperatureMax")]
    pub temperature_max: Option<f64>,
    #[serde(rename = "temperatureMaxTime")]
    pub temperature_max_time: Option<i64>,
    #[serde(rename = "apparentTemperatureHigh")]
    pub apparent_temperature_high: Option<f64>,
    #[serde(rename = "apparentTemperatureHighTime")]
    pub apparent_temperature_high_time: Option<i64>,
    #[serde(rename = "apparentTemperatureLow")]
    pub apparent_temperature_low: Option<f64>,
    #[serde(rename = "apparentTemperatureLowTime")]
    pub apparent_temperature_low_time: Option<i64>,
    #[serde(rename = "precipIntensityMax")]
    pub precip_intensity_max: Option<f64>,
    #[serde(rename = "precipIntensityMaxTime")]
    pub precip_intensity_max_time: Option<i64>,
    #[serde(rename = "precipIntensityMin")]
    pub precip_intensity_min: Option<f64>,
    #[serde(rename = "precipIntensityMinTime")]
    pub precip_intensity_min_time: Option<i64>,
    #[serde(rename = "windGustTime")]
    pub wind_gust_time: Option<i64>,

    #[serde(rename = "uvIndexTime")]
    pub uv_index_time: Option<i64>,
    #[serde(rename = "dawnTime")]
    pub dawn_time: Option<i64>,
    #[serde(rename = "duskTime")]
    pub dusk_time: Option<i64>,
    #[serde(rename = "currentDayIce")]
    pub current_day_ice: Option<f64>,
    #[serde(rename = "currentDayLiquid")]
    pub current_day_liquid: Option<f64>,
    #[serde(rename = "currentDaySnow")]
    pub current_day_snow: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Alert {
    pub title: String,
    pub regions: Vec<String>,
    pub severity: String,
    pub time: i64,
    pub expires: i64,
    pub description: String,
    pub uri: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Flags {
    pub units: String,
    pub version: String,
    #[serde(rename = "nearest-station")]
    pub nearest_station: Option<f64>,
    pub sources: Vec<String>,
    #[serde(rename = "sourceIDX")]
    pub source_idx: Option<Vec<SourceIdx>>,
    #[serde(rename = "processTime")]
    pub process_time: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SourceIdx {
    pub x: i64,
    pub y: i64,
}
