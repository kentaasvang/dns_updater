use serde::Deserialize;
use serde_json::*;
use std::{fs, io::Read};

#[derive(Deserialize, Debug)]
struct Settings {
    cf_api_key: String,
}

fn get_settings(path: String) -> Settings {
    let content = fs::read_to_string(path).expect("settings file not found");

    let settings: Settings = serde_json::from_str(&content).expect("Couldn't deserialize settings file");
    settings
}

fn main() {
    // Read config
    let config_path = String::from("settings.json");
    let config = get_settings(config_path);
    println!("{:?}", config);

    // store domains in db
    // store in file temporary
    // NOTE: should set up directory /opt/dns_updater and store data (sqlite3 or flat file)
    println!("store data before continuing");

    //fs::create_dir_all("/opt/dns_updater").expect("failed to create dns_updater folder");
    // let data_file = fs::File::create_new("/opt/dns_updater/").expect("path already exists");

    // Get all A-records from cloudflare
    // Store records in database
    // Sleep for N seconds
    // Get public IP address
    // Get active records from DB
    // Update record in CF if not same as public IP
    //

    println!("FINISHED");
}
