pub mod models;
pub mod schema;

use self::models::{Equipment, NewEquipments};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_equipment(conn: &mut SqliteConnection, name: &str, km: &i32) -> Equipment {
    use crate::schema::equipments;

    let new_equipment = NewEquipments { name, km };

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
