#![allow(dead_code, unused_imports)]

use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Domain {
    name: String,
    active: bool
}

#[derive(Debug)]
struct Config {
    domains: Vec<Domain>
}


fn read_config(config_path: String) -> Config {
    let mut domains: Vec<Domain> = Vec::new();

    let mut rdr = csv::Reader::from_path(config_path)
       .expect("no file found");

    for result in rdr.deserialize() {
        let domain: Domain = result.expect("No config found.");
        domains.push(domain)
    }
    
    let config = Config { domains };
    config
}

fn main() {

    // Read config 
    let config_path = String::from("config.csv");
    let config = read_config(config_path);
    println!("{:?}", config);
   
    // store domains in db

    // Get all A-records from cloudflare
    // Store records in database
    // Sleep for N seconds
    // Get public IP address
    // Get active records from DB
    // Update record in CF if not same as public IP
    //
    
    println!("FINISHED");
}
