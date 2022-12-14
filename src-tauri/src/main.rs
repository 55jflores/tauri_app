
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{Menu, CustomMenuItem, MenuItem};
use serde::{Serialize, Deserialize};
use std::env;

// Struct that holds APOD JSON 
#[derive(Debug, Serialize, Deserialize)]
struct Nasa {
    date: String,
    explanation: String,
    hdurl: String,
    media_type: String,
    service_version: String,
    title: String,
    url: String
}

#[tokio::main]
async fn apod() -> Result<Vec<String>, reqwest::Error> {
    // Get request to grab APOD
    let nasa_apod: Nasa = reqwest::Client::new()
    .get("https://api.nasa.gov/planetary/apod?api_key=API_KEY_GIVEN")
    .send()
    .await?
    .json()
    .await?;

    // Vector holding title, picture url and explanation
    let vec_apod = vec![nasa_apod.title,nasa_apod.hdurl, nasa_apod.explanation];
    
    // Return result
    Ok(vec_apod)
}
#[tauri::command]
fn grab_pic() -> Vec<String> {
    // Grabbing result vector with data
    let img_vec = apod();
    // Unwrapping and converting to vector of strings
    let react_vec: Vec<String> = img_vec.unwrap();
    
    react_vec
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![grab_pic])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
