// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracker::create_equipment;

use tracker::create_log;
use tracker::create_person;
use tracker::create_revision;
use tracker::establish_connection;
use tracker::find_all_equipments;
use tracker::find_all_logs;
use tracker::find_all_persons;
use tracker::find_all_revisions;
use tracker::models::Log;
use tracker::models::Revision;
use tracker::models::{Equipment, Person};
use tracker::update_equipment_km_by_id;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//tauri command to call create equipment with params from the frontend
#[tauri::command]
fn add_equipment(name: &str, km: i32) -> Equipment {
    let connection = &mut establish_connection();
    return create_equipment(connection, name, &km);
}

#[tauri::command]
fn add_person(name: &str) -> Person {
    let connection = &mut establish_connection();
    return create_person(connection, name);
}

#[tauri::command]
fn add_log(equipment_id: i32, person_id: i32, description: String, km: i32,job: String) -> Log {
    let connection = &mut establish_connection();
    update_equipment_km_by_id(connection, &equipment_id, &km);
    return create_log(connection, &equipment_id, &person_id, &description, &km, &job);
}

#[tauri::command]
fn add_revision(equipment_id: i32, tipo: String, target: String) -> Revision {
    let connection = &mut establish_connection();
    return create_revision(connection, &equipment_id, &tipo, &target);
}


#[tauri::command]
fn get_equipments() -> Result<String, String> {
    let connection = &mut establish_connection();
    let results = find_all_equipments(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}

#[tauri::command]
fn get_persons() -> Result<String, String> {
    let connection = &mut establish_connection();
    let results = find_all_persons(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}

#[tauri::command]
fn get_logs() -> Result<String, String> {
    let connection = &mut establish_connection();
    let results = find_all_logs(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}

#[tauri::command]
fn get_revisions() -> Result<String, String> {
    let connection = &mut establish_connection();
    let results = find_all_revisions(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_equipments,
            add_equipment,
            add_person,
            get_persons,
            add_log,
            get_logs,
            add_revision,
            get_revisions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
