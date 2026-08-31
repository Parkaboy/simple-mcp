use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct Forecast {
    pub(crate) city: String,
    pub(crate) temp_c: f64,
    pub(crate) summary: String,
}

#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<Location>>,
}

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature_2m: f64,
    weather_code: i32,
}

pub(crate) async fn fetch_upstream(city: &str) -> anyhow::Result<Forecast> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let location = client
        .get("https://geocoding-api.open-meteo.com/v1/search")
        .query(&[
            ("name", city),
            ("count", "1"),
            ("language", "en"),
            ("format", "json"),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<GeocodingResponse>()
        .await?
        .results
        .and_then(|mut locations| locations.pop())
        .ok_or_else(|| anyhow::anyhow!("city not found"))?;

    let weather = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&[
            ("latitude", location.latitude.to_string()),
            ("longitude", location.longitude.to_string()),
            ("current", "temperature_2m".to_string()),
            ("current", "weather_code".to_string()),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<WeatherResponse>()
        .await?;

    Ok(Forecast {
        city: location.name,
        temp_c: weather.current.temperature_2m,
        summary: weather_summary(weather.current.weather_code).to_string(),
    })
}

fn weather_summary(code: i32) -> &'static str {
    match code {
        0 => "clear sky",
        1..=3 => "partly cloudy",
        45 | 48 => "foggy",
        51..=67 => "rainy",
        71..=77 => "snowy",
        80..=82 => "rain showers",
        85 | 86 => "snow showers",
        95 | 96 | 99 => "thunderstorms",
        _ => "unknown conditions",
    }
}
