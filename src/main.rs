mod data;
mod file;
mod parse;
mod utils;

use std::env;
use std::error::Error;

use data::Secret;

use crate::file::{get_secret_file_path, load_secret, write_json_file};
use crate::parse::parse_arguments;
use crate::utils::show_data;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut secret: Secret = load_secret().await?;
    let args: Vec<String> = env::args().collect();

    parse_arguments(&args, &mut secret)?;

    let file_path_str = get_secret_file_path().to_string_lossy().to_string();
    write_json_file(&file_path_str, &secret).await?;
    show_data(&secret).await?;

    Ok(())
}
