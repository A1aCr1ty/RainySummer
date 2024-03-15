use crate::data::{CurrentWeather, DailyData, DailyWeatherResult, NowWeatherResult, Secret};
use std::error::Error;

const BASE_URL: &str = "https://api.seniverse.com/v3/weather";

async fn get_current_weather(
    secret: &Secret,
) -> Result<CurrentWeather, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}/current.json?key={}&location={}&language={}&unit=metric",
        BASE_URL, secret.api_key, secret.location, secret.language
    );

    println!("{}", url);
    let res: reqwest::Response = reqwest::get(&url).await?;
    if res.status().is_success() {
        let body: String = res.text().await?;
        let weather_data: serde_json::Value = serde_json::from_str(&body)?;
        if let Some(result) = weather_data["result"].get(0) {
            let result: NowWeatherResult = serde_json::from_value(result.clone())?;
            let current_weather: CurrentWeather = CurrentWeather {
                text: result.now.text,
                temperature: result.now.temperature,
            };
            return Ok(current_weather);
        } else {
            println!("No results found!");
        }
    } else {
        println!("Request failed with status {}", res.status());
    }
    Err("Error occurred during weather retrieval".into())
}
async fn get_future_weather(secret: &Secret) -> Result<Vec<DailyData>, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}/daily.json?key={}&location={}&language={}&unit=metric",
        BASE_URL, secret.api_key, secret.location, secret.language
    );
    let res: reqwest::Response = reqwest::get(&url).await?;
    if res.status().is_success() {
        let body: String = res.text().await?;
        let weather_data: serde_json::Value = serde_json::from_str(&body)?;
        if let Some(result) = weather_data["result"].get(0) {
            let result: DailyWeatherResult = serde_json::from_value(result.clone())?;
            let daily_weather: Vec<DailyData> = result.daily[1..].to_vec();
            return Ok(daily_weather);
        } else {
            println!("No results found!\n");
        }
    } else {
        println!("Reqwest failed with status {}", res.status());
    }
    Err("Error occurred during weather retrieval".into())
}

pub async fn show_data(secret: &Secret) -> Result<(), Box<dyn std::error::Error>> {
    let current_weather: CurrentWeather = get_current_weather(secret).await?;
    let daily_weather: Vec<DailyData> = get_future_weather(secret).await?;

    let loaction_en_to_zh: String = match secret.location.as_str() {
        "GuangZhou" => "广州".to_string(),
        "ShenZhen" => "深圳".to_string(),
        "ShangHai" => "上海".to_string(),
        "BeiJing" => "北京".to_string(),
        _ => secret.location.clone(),
    };

    if secret.language == "zh-Hans" {
        println!();
        print!("{} (￣︶￣)↗ | ", loaction_en_to_zh);
        print!("{} | ", current_weather.text);
        println!("{}°C", current_weather.temperature);

        for daily_data in &daily_weather {
            println!("·");
            print!("{} | ", daily_data.date);
            print!("白天：{} | ", daily_data.text_day);
            print!("夜晚：{} | ", daily_data.text_night);
            print!("{}°C ~ ", daily_data.low);
            println!("{}°C", daily_data.high);
        }
    } else if secret.language == "en" {
        println!();
        print!("{} (￣︶￣)↗ | ", secret.location);
        print!("{} | ", current_weather.text);
        println!("{}°C", current_weather.temperature);

        for daily_data in &daily_weather {
            println!("·");
            print!("{} | ", daily_data.date);
            print!("Day: {} | ", daily_data.text_day);
            print!("Night: {} | ", daily_data.text_night);
            print!("{}°C ~ ", daily_data.low);
            println!("{}°C", daily_data.high);
        }
    }

    Ok(())
}
