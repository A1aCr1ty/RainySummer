use std::io::Stdout;

use serde::{Deserialize, Serialize};
use tui::backend::CrosstermBackend;

pub type CrossTerminal = tui::Terminal<CrosstermBackend<Stdout>>;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    id: String,
    name: String,
    country: String,
    path: String,
    timezone: String,
    timezone_offset: String,
}
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Now {
    pub text: String,
    pub code: String,
    pub temperature: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NowWeatherResult {
    location: Location,
    pub now: Now,
    last_update: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub text: String,
    pub temperature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyData {
    pub date: String,
    pub text_day: String,
    pub code_day: String,
    pub text_night: String,
    pub code_night: String,
    pub high: String,
    pub low: String,
    pub rainfall: String,
    pub precip: String,
    pub wind_direction: String,
    pub wind_direction_degree: String,
    pub wind_speed: String,
    pub wind_scale: String,
    pub humidity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyWeatherResult {
    location: Location,
    pub daily: Vec<DailyData>,
    last_update: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    pub api_key: String,
    pub location: String,
    pub language: String,
}
impl Default for Secret {
    fn default() -> Self {
        Self {
            api_key: "SkyWErkPwye-1C6wv".to_string(),
            location: "GuangZhou".to_string(),
            language: "en".to_string(),
        }
    }
}
