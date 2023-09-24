pub mod models;
pub mod schema;

use self::models::{Equipment, NewEquipment};
use self::models::{Log, NewLog};
use self::models::{Person, NewPerson};
use diesel::prelude::*;
use dotenvy::dotenv;
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
pub fn find_equipment_by_id(conn: &mut SqliteConnection, post_id: i32) -> Equipment {
    use self::schema::equipments::dsl::*;

    let equipment = equipments
        .find(post_id)
        .select(Equipment::as_select())
        .first(conn)
        .expect("Error loading equipment");
    return equipment
}
pub fn update_equipment_km_by_id(conn: &mut SqliteConnection, equipment_id: &i32, new_km: &i32) -> Equipment {
    use self::schema::equipments::dsl::*;

    let equipment = diesel::update(equipments.find(equipment_id))
        .set(km.eq(new_km))
        .get_result(conn)
        .expect("Error loading equipment");
    return equipment
}
//LOGS
pub fn create_log(
    conn: &mut SqliteConnection, equipment_id: &i32, person_id: &i32, description: &String, km: &i32, job: &String ) -> Log {
    use crate::schema::logs;

    let new_log = NewLog { equipment_id, person_id, description, km, job };

    let log = diesel::insert_into(logs::table)
        .values(&new_log)
        .returning(Log::as_returning())
        .get_result(conn)
        .expect("Error saving new equipment");

    return log;
}
pub fn find_all_logs(conn: &mut SqliteConnection) -> Vec<Log> {
    use self::schema::logs::dsl::*;

    let results = logs
        .select(Log::as_select())
        .load(conn)
        .expect("Error loading logs");
    return results;
}
//PERSONS
pub fn create_person(conn: &mut SqliteConnection, name: &str) -> Person {
    use crate::schema::persons;

    let new_person = NewPerson { name };

    let person = diesel::insert_into(persons::table)
        .values(&new_person)
        .returning(Person::as_returning())
        .get_result(conn)
        .expect("Error saving new equipment");

    return person;
}
pub fn find_all_persons(conn: &mut SqliteConnection) -> Vec<Person> {
    use self::schema::persons::dsl::*;

    let results = persons
        .select(Person::as_select())
        .load(conn)
        .expect("Error loading persons");
    return results;
}
//USERS
