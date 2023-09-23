pub mod models;
pub mod schema;

use self::models::{Equipment, NewEquipment};
use self::models::{Log, NewLog};
use diesel::prelude::*;
use dotenvy::dotenv;
use schema::logs::{equipment_id, description, previous_km, new_km};
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
//EQUIPMENT
pub fn create_equipment(conn: &mut SqliteConnection, name: &str, km: &i32) -> Equipment {
    use crate::schema::equipments;

    let new_equipment = NewEquipment { name, km };

    let eq = diesel::insert_into(equipments::table)
        .values(&new_equipment)
        .returning(Equipment::as_returning())
        .get_result(conn)
        .expect("Error saving new equipment");

    return eq;
}

pub fn find_all_equipments(conn: &mut SqliteConnection) -> Vec<Equipment> {
    use self::schema::equipments::dsl::*;

    let results = equipments
        .select(Equipment::as_select())
        .load(conn)
        .expect("Error loading equipments");
    return results;
}
//LOGS
pub fn create_log(
    conn: &mut SqliteConnection, equipment_id: &i32, person_id: &i32, description: &String, previous_km: &i32, new_km: &i32, ) -> Log {
    use crate::schema::logs;

    let new_log = NewLog { equipment_id, person_id, description, previous_km, new_km };

    let log = diesel::insert_into(logs::table)
        .values(&new_log)
        .returning(Log::as_returning())
        .get_result(conn)
        .expect("Error saving new equipment");

    return log;
}
//PERSONS
//USERS
