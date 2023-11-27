// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use diesel_migrations::embed_migrations;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use track_tor::create_brand;
use track_tor::create_equipment;
use track_tor::create_log;
use track_tor::create_model;
use track_tor::create_person;
use track_tor::create_revision;
use track_tor::establish_connection;
use track_tor::create_manual;
use track_tor::find_all_brands;
use track_tor::find_all_manuals;
use track_tor::find_all_equipments;
use track_tor::find_all_logs;
use track_tor::find_all_models;
use track_tor::find_all_persons;
use track_tor::find_all_revisions;
use track_tor::models::Brand;
use track_tor::models::Log;
use track_tor::models::Model;
use track_tor::models::Revision;
use track_tor::models::Manual;
use track_tor::models::{Equipment, Person};
use track_tor::update_equipment;
use track_tor::update_equipment_km_by_id;
use track_tor::update_log;
use track_tor::update_log_created_at_by_id;
use track_tor::update_model;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//tauri command to call create equipment with params from the frontend
//EQUIPMENT

#[tauri::command]
fn get_equipments() -> Result<String, String> {
    let connection = &mut establish_connection();
    let results = find_all_equipments(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}

#[tauri::command]
fn put_equipment(
    equipment_id: i32,
    name: String,
    km: i32,
    model_id: i32,
    nserial: String,
    notes: String,
    file_path: String,
) -> Equipment {
    let connection = &mut establish_connection();
    return update_equipment(
        connection,
        &equipment_id,
        &name,
        &km,
        &model_id,
        &nserial,
        &notes,
        &file_path,
    );
}

#[tauri::command]
fn add_equipment(
    name: String,
    km: i32,
    model_id: i32,
    nserial: String,
    notes: String,
    file_path: String,
) -> Equipment {
    let connection = &mut establish_connection();
    return create_equipment(
        connection, &name, &km, &model_id, &nserial, &notes, &file_path,
    );
}

//PERSONS
#[tauri::command]
fn add_person(name: &str) -> Person {
    let connection = &mut establish_connection();
    return create_person(connection, name);
}

#[tauri::command]
fn get_persons() -> Result<String, String> {
    let connection = &mut establish_connection();
    let results = find_all_persons(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}

//LOGS
#[tauri::command]
fn add_log(equipment_id: i32, person_id: i32, description: String, km: i32, job: String) -> Log {
    let connection = &mut establish_connection();
    update_equipment_km_by_id(connection, &equipment_id, &km);
    return create_log(
        connection,
        &equipment_id,
        &person_id,
        &description,
        &km,
        &job,
    );
}

#[tauri::command]
fn edit_log(
    log_id: i32,
    equipment_id: i32,
    person_id: i32,
    description: String,
    km: i32,
    job: String,
    new_created_at: String,
) -> Log {
    let connection = &mut establish_connection();
    return update_log(
        connection,
        &log_id,
        &equipment_id,
        &person_id,
        &description,
        &km,
        &job,
        &new_created_at,
    );
}
#[tauri::command]
fn edit_log_date(log_id: i32, new_date: String) -> Log {
    let connection = &mut establish_connection();
    return update_log_created_at_by_id(connection, &log_id, &new_date);
}
#[tauri::command]
fn get_logs() -> Result<String, String> {
    let connection = &mut establish_connection();
    let results = find_all_logs(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}
//REVISIONS
#[tauri::command]
fn add_revision(equipment_id: i32, tipo: String, target: String) -> Revision {
    let connection = &mut establish_connection();
    return create_revision(connection, &equipment_id, &tipo, &target);
}

#[tauri::command]
fn get_revisions() -> Result<String, String> {
    let connection = &mut establish_connection();
    let results = find_all_revisions(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}

//BRANDS
#[tauri::command]
fn add_brand(name: String) -> Brand {
    let connection = &mut establish_connection();
    return create_brand(connection, &name);
}

#[tauri::command]
fn get_brands() -> Result<String, String> {
    let connection = &mut establish_connection();
    let results = find_all_brands(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}

//MODELS
#[tauri::command]
fn add_model(name: String, brand_id: i32) -> Model {
    let connection = &mut establish_connection();
    return create_model(connection, &name, &brand_id);
}

#[tauri::command]
fn add_manual(name:String, equipment_id: i32, file_path: String) -> Manual {
    let connection = &mut establish_connection();
    return create_manual(connection, &name, &file_path, &equipment_id);
}

#[tauri::command]
fn put_model(model_id: i32, new_name: String, new_brand_id: i32) -> Model {
    let connection = &mut establish_connection();
    return update_model(connection, &model_id, &new_name, &new_brand_id);
}

#[tauri::command]
fn get_models() -> Result<String, String> {
    let connection = &mut establish_connection();
    let results = find_all_models(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}

#[tauri::command]
fn get_manuals() -> Result<String, String> {
    let connection = &mut establish_connection();
    let results = find_all_manuals(connection);
    let json = serde_json::to_string(&results).unwrap();
    return Ok(json);
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn main() {
    let _ = &mut establish_connection().run_pending_migrations(MIGRATIONS);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_equipments,
            add_equipment,
            add_manual,
            put_equipment,
            put_model,
            edit_log,
            add_person,
            get_persons,
            add_log,
            get_logs,
            edit_log_date,
            add_revision,
            get_revisions,
            add_brand,
            get_brands,
            add_model,
            get_manuals,
            get_models
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
