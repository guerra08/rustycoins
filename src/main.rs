use std::env;
use config::{Source};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse ARGS
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Missing ARGS: base_currency [USD] wanted_currency [BRL]");
        std::process::exit(1)
    }
    let base_currency = &args[1].to_uppercase();
    let wanted_currency = &args[2].to_uppercase();

    let currency_key = base_currency.to_owned() + "_" + wanted_currency;

    // Load settings
    let mut settings_path = dirs::config_dir().unwrap();
    settings_path.push("./rustycoins/settings.toml");
    let settings_map = config::File::from(settings_path).collect().unwrap();

    // Send and process request
    let url =
        format!("https://free.currconv.com/api/v7/convert?apiKey={api_key}&q={query}&compact=ultra",
            api_key = settings_map.get("api_key").unwrap(),
            query = currency_key
        );

    let res = reqwest::get(url)
        .await?
        .text()
        .await?;
    println!("{}", res);
    Ok(())
}
