use std::error::Error;

use crate::data::{Secret};


pub fn parse_arguments(args: &[String], secret: &mut Secret) -> Result<(), Box<dyn Error>> {
    let mut index: usize = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--location" => {
                parse_location_args(args, &mut index, secret);
            }
            "--language" => {
                parse_language_args(args, &mut index, secret);
            }
            "--help" => {
                print_help();
                return Ok(());
            }
            _ => {
                eprintln!("Error: Unknown command. Use --help for usage information.");
                return Ok(());
            }
        }
    }

    return Ok(());
}

fn parse_location_args(
    args: &[String],
    index: &mut usize,
    secret: &mut Secret,
) -> Result<(), Box<dyn Error>> {
    if *index + 1 > args.len() {
        eprintln!("Error: Missing location argument.");
        return Ok(());
    }
    secret.location = args[*index + 1].clone();
    *index += 2;
    return Ok(());
}

fn parse_language_args(
    args: &[String],
    index: &mut usize,
    secret: &mut Secret,
) -> Result<(), Box<dyn Error>> {
    if *index + 1 > args.len() {
        eprintln!("Error: Missing language argument.");
        return Ok(());
    }
    secret.language = args[*index + 1].clone();
    *index += 2;
    return Ok(());
}
fn print_help() {
    println!("Usage:");
    println!("  --location <location> : Set the location in the secret.json file.");
    println!("  --language <language> : Set the language in the secret.json file. Only supports Chinese (zh-Hans) and English (en).");
    println!("  --help                : Show this help message.");
    println!();
}
