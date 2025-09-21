use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: weather_api_cli <latitude> <longitude>");
        std::process::exit(1);
    }
    let lat: &String = &args[1];
    let long: &String = &args[1];

    print!("Debug! :: {} || {}", lat, long);
    println!("Hello, world! Rust");
    Ok(())
}
