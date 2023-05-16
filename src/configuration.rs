use std::fs::{read_to_string};
use serde_json::{from_str, Value};

pub fn get_config_value(key: &str) -> String {
    let config_path = "config/config.json";
    
    // read the file at that path 
    let json_string = read_to_string(config_path).expect("Cannot read config file");
    let json_value: Value = from_str(json_string.as_str()).expect("Unable to read JSON");
    let raw_value: String = json_value.get(key).expect("Cannot access specified value").to_string();
    let value = raw_value.chars().filter(|c| *c != '"').collect::<String>();
    value
}

pub fn recently_played_path() -> String {
    format!("{}recently_played.json", get_assets_dir())
}

pub fn get_assets_dir() -> String {
    get_config_value("assets_path")
}


pub fn get_image_dir() -> String {
    format!("{}Images/", get_config_value("assets_path"))
}
