use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct Driver {
    pub name: String,
    pub website_url: String,
    pub team: String,
    pub short_name: String,
}

pub fn get_drivers() -> Vec<Driver> {
    let json_file = read_to_string("./drivers.json").unwrap();
    let json_str = json_file.as_str();
    let drivers: Vec<Driver> = serde_json::from_str(json_str).unwrap();

    return drivers;
}
