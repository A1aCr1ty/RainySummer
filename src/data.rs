use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    id: String,
    name: String,
    country: String,
    path: String,
    timezone: String,
    timezone_offset: String,
}
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
