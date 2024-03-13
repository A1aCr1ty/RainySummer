mod data;
mod parse;

use std::{
    env::{self, args},
    error::Error,
};

use data::Secret;

use crate::parse::parse_arguments;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut secret = Secret::default();
    let args: Vec<String> = env::args().collect();
    let _ = parse_arguments(&args, &mut secret);
    println!("{:#?}", secret);
    Ok(())
}
