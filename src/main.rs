use serde::Deserialize;
use simple_cloudflare_api::list_zones;
use std::fs;

#[derive(Deserialize, Debug)]
struct Settings {
    cf_api_key: String,
}

fn get_settings(path: String) -> Settings {
    let content = fs::read_to_string(path).expect("settings file not found");

    let settings: Settings =
        serde_json::from_str(&content).expect("Couldn't deserialize settings file");
    settings
}

#[tokio::main]
async fn main() {
    // Read config
    let config_path = String::from("settings.json");
    let config = get_settings(config_path);

    // Get all A-records from cloudflare
    let zones = list_zones(config.cf_api_key).await;
    println!("{:?}", zones);

    // Store records
    // Sleep for N seconds
    // Get public IP address
    // Get active records from DB
    // Update record in CF if not same as public IP

    println!("FINISHED");
}
