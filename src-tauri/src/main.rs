// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracker::create_equipment;

use tracker::establish_connection;
use tracker::find_all_equipments;
use serde::Serialize;
use tracker::models::Equipment;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//tauri command to call create equipment with params from the frontend
#[tauri::command]
fn add_equipment(name: &str, km: i32) -> Equipment {
    let connection = &mut establish_connection();
    return create_equipment(connection, name, &km);
}

#[tauri::command]
fn get_equipments() -> Result<String, String> {
    print!("AAAAAAAAAAAAA");
    let connection = &mut establish_connection();
    let results = find_all_equipments(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_equipments, add_equipment])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
