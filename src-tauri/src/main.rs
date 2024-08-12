// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize, Debug)]
struct Article {
    id: u32,
    title: String,
    url: String,
    image_url: String,
    news_site: String,
    summary: String,
    published_at: String,
    updated_at: String,
    featured: bool,
    // launches: Vec<String>,
    // events: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    count: u32,
    // next: Option<String>,
    // previous: Option<String>,
    results: Vec<Article>,
}

#[tauri::command]
async fn fetch_articles(limit: u32, offset: u32) -> Result<ApiResponse, String> {
    let api_url = format!(
        "https://api.spaceflightnewsapi.net/v4/articles/?limit={}&offset={}",
        limit, offset
    );

    // let body = reqwest::get(&api_url)
    //     .await
    //     .map_err(|e| e.to_string())?
    //     .text()
    //     .await;

    // println!("body = {body:?}");

    let response = reqwest::get(&api_url)
        .await
        .map_err(|e| e.to_string())?
        .json::<ApiResponse>()
        .await
        .map_err(|e| e.to_string())?;

    
    // println!("count: {}", response.count);

    Ok(response)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,fetch_articles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
