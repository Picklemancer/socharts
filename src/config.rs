use serde::Deserialize;
use serde_json;
use std::env;
use std::fs;

#[derive(Deserialize)]
pub struct Property {
    pub label: String,
    pub key: String,
    pub color: String,
}

#[derive(Deserialize)]
pub struct Graph {
    pub label: String,
    pub properties: Vec<Property>,
}

#[derive(Deserialize)]
pub struct Config {
    pub sample_duration: f32,
    pub graphs: Vec<Graph>,
}

pub fn get_config() -> Config {
    // current_dir().unwrap() -> current_dir().expect("current directory read failed")
    let file = fs::read(env::current_dir().unwrap().display().to_string() + "/config.json")
        .expect("config.json missing");

    serde_json::from_slice(&file).expect("config.json not json")
}
