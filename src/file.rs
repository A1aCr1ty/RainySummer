use crate::data::Secret;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub async fn load_secret() -> Result<Secret, Box<dyn Error>> {
    let file_path: PathBuf = get_secret_file_path();
    let file_path_str: std::borrow::Cow<'_, str> = file_path.to_string_lossy();
    read_json_file(&file_path_str).await
}

pub fn get_secret_file_path() -> PathBuf {
    let mut home_dir: PathBuf = dirs::home_dir().expect("Failed to get home directory");
    home_dir.push("secret.json");
    home_dir
}

pub async fn reset_json_file() -> Result<(), Box<dyn std::error::Error>> {
    let default_api_key: String = "SJUmznOwVrH_ly-Om".to_string();
    let default_location: String = "ChengDu".to_string();
    let default_language: String = "en".to_string();
    let secret_date: Secret = Secret {
        api_key: default_api_key.clone(),
        location: default_location.clone(),
        language: default_language.clone(),
    };

    let json_string: String = serde_json::to_string_pretty(&secret_date)?;
    let mut home_dir: std::path::PathBuf = dirs::home_dir().expect("Failed to get home directory");
    home_dir.push("secret.json");

    let mut file: File = File::create(home_dir)?;
    file.write_all(json_string.as_bytes())?;
    println!("Data successfully written to secret.json.");
    return Ok(());
}

pub async fn read_json_file(file_path: &str) -> Result<Secret, Box<dyn std::error::Error>> {
    if !Path::new(file_path).exists() {
        reset_json_file().await?;
    }

    let mut file: File = File::open(file_path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    let secret: Secret = match serde_json::from_str(&contents) {
        Ok(s) => s,
        Err(_) => {
            reset_json_file().await?;
            Secret::default()
        }
    };

    Ok(secret)
}

pub async fn write_json_file(
    file_path: &str,
    secret: &Secret,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: File = File::create(file_path)?;
    let mut secret_json: String = serde_json::to_string_pretty(secret)?;
    let written_bytes: usize = file.write(secret_json.as_bytes())?;
    if written_bytes > 0 {
        Ok(())
    } else {
        println!("Failed to write data to the file. \n");
        Ok(())
    }
}
