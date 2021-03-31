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
    let base_currency = &args[1];
    let wanted_currency = &args[2];
    let url = "https://api.exchangeratesapi.io/latest?base=".to_owned() + base_currency;
    let res: ResponseType = reqwest::get(url)
        .await?
        .json()
        .await?;
    println!("{}", res.rates.get(wanted_currency).unwrap());
    Ok(())
}
