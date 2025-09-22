use reqwest::blocking::Client;
use std::env;
use std::error::Error;

/*
https://api.openweathermap.org/data/2.5/weather?lat=-0.2252806&lon=109.6446236&appid=env
*/

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("MY_WEATHER_API_KEY")
        .expect("Please set the MY_WEATHER_API_KEY environment variable");

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: weather_api_cli <latitude> <longitude>");
        std::process::exit(1);
    }
    let lat: &String = &args[1];
    let long: &String = &args[1];

    let client = Client::new();
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        lat, long, api_key
    );

    print!("Debug! :: {} || {}", lat, long);
    println!("Hello, world! Rust");
    Ok(())
}
