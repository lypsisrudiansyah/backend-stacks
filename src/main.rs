use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;
use std::error::Error;

/*
https://api.openweathermap.org/data/2.5/weather?lat=-0.2252806&lon=109.6446236&appid=env
*/

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    rain: Option<Rain>, // optional, because sometimes "rain" may not exist
    sys: Sys,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
}

// Special case: "1h" is not a valid Rust identifier,
// so we use #[serde(rename = "1h")]
#[derive(Debug, Deserialize)]
struct Rain {
    #[serde(rename = "1h")]
    one_hour: f64,
}

#[derive(Debug, Deserialize)]
struct Sys {
    country: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("MY_WEATHER_API_KEY")
        .expect("Please set the MY_WEATHER_API_KEY environment variable");

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: weather_api_cli <latitude> <longitude>");
        std::process::exit(1);
    }
    let lat: &String = &args[1];
    let long: &String = &args[2];

    let client = Client::new();
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        lat, long, api_key
    );

    // let raw = client.get(&url).send()?.text()?;
    // println!("Raw response:\n{}", raw);

    let weather_data: WeatherResponse = client.get(&url).send()?.json()?;

    if let Some(country) = &weather_data.sys.country {
        println!("Place: {}, Country {}", weather_data.name, country)
    } else {
        println!("Country not defined!")
    };
    println!(
        "Temperature: {:.2}°C / {:.2}°F",
        weather_data.main.temp - 273.15,
        (weather_data.main.temp - 273.15) * 9.0 / 5.0 + 32.0
    );

    println!("------------=====---------");
    println!(
        "City: {}, Country: {}, Temp: {}°C, Weather: {}",
        weather_data.name,
        // weather_data.sys.country,
        weather_data.sys.country.as_deref().unwrap_or("N/A"),
        weather_data.main.temp,
        weather_data
            .weather
            .first()
            .map(|w| &w.description)
            .unwrap_or(&"N/A".to_string()),
    );

    print!("Debug! :: {} || {}", lat, long);
    // println!("Hello, world! Rust");
    Ok(())
}
