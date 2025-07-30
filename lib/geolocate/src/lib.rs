use anyhow::Error;
use serde::Deserialize;

const IPAPI_URL: &str = "https://api.ipgeolocation.io/v2/ipgeo";

#[derive(Default, Debug, Deserialize, Clone)]
pub struct GeoResponse {
    pub ip: String,
    pub location: Location,
    pub country_metadata: CountryMetadata,
    pub currency: Currency,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Location {
    pub continent_code: String,
    pub continent_name: String,
    pub country_code2: String,
    pub country_code3: String,
    pub country_name: String,
    pub country_name_official: String,
    pub country_capital: String,
    pub state_prov: String,
    pub state_code: String,
    pub district: String,
    pub city: String,
    pub zipcode: String,
    pub latitude: String,
    pub longitude: String,
    pub is_eu: bool,
    pub country_flag: String,
    pub geoname_id: String,
    pub country_emoji: String,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct CountryMetadata {
    pub calling_code: String,
    pub tld: String,
    pub languages: Vec<String>,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

pub async fn get_geo_location() -> Result<GeoResponse, Error> {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_net::http::Request;

        let api_key = env!("IPLOCATE_API_KEY");
        let url = format!("{IPAPI_URL}?apiKey={api_key}");

        Ok(Request::get(&url)
            .send()
            .await?
            .json::<GeoResponse>()
            .await?)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let api_key = std::env::var("IPLOCATE_API_KEY")?;
        let url = format!("{IPAPI_URL}?apiKey={api_key}");
        Ok(reqwest::get(url).await?.json::<GeoResponse>().await?)
    }
}
