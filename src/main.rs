use std::env;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
struct ResponseType{
    rates: HashMap<String, f32>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Missing ARGS: base_currency [USD] wanted_currency [BRL]");
        std::process::exit(1)
    }
    let base_currency = &args[1].to_uppercase();
    let wanted_currency = &args[2].to_uppercase();
    let url = "https://api.exchangeratesapi.io/latest?base=".to_owned() + base_currency;
    let res: ResponseType = reqwest::get(url)
        .await?
        .json()
        .await?;
    let output = res.rates.get(wanted_currency).expect("Invalid currency.");
    println!("{}", output);
    Ok(())
}
