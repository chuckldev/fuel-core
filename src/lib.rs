use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FuelEntry {
    pub date: String,
    pub store: String,
    pub pump: String,
    pub gallons: f64,
    pub cost: f64,
    pub mileage: f64,
}

#[wasm_bindgen]
pub fn process_entry(json: &str) -> String {
    let entry: FuelEntry = serde_json::from_str(json).unwrap();
    serde_json::to_string(&entry).unwrap()
}