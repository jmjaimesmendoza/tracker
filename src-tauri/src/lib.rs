pub mod models;
pub mod schema;

use crate::models::{NewBrand, NewModel, Brand, Model};

use self::models::{Equipment, NewEquipment};
use self::models::{Log, NewLog};
use self::models::{Person, NewPerson};
use diesel::prelude::*;
use dotenvy::dotenv;
use models::Revision;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
//BRANDS
pub fn create_brand(conn: &mut SqliteConnection, name: &String) -> Brand {
    use crate::schema::brands;

    let new_brand = NewBrand { name };

    let brand = diesel::insert_into(brands::table)
        .values(&new_brand)
        .returning(Brand::as_returning())
        .get_result(conn)
        .expect("Error saving new brand");

    return brand;
}

pub fn find_all_brands(conn: &mut SqliteConnection) -> Vec<Brand> {
    use self::schema::brands::dsl::*;

    let results = brands
        .select(Brand::as_select())
        .load(conn)
        .expect("Error loading brands");
    return results;
}
//MODELS
pub fn create_model(conn: &mut SqliteConnection, name: &String, brand_id: &i32) -> Model {
    use crate::schema::models;

    let new_model = NewModel { name, brand_id };

    let model = diesel::insert_into(models::table)
        .values(&new_model)
        .returning(Model::as_returning())
        .get_result(conn)
        .expect("Error saving new model");

    return model;
}

pub fn find_all_models(conn: &mut SqliteConnection) -> Vec<Model> {
    use self::schema::models::dsl::*;
    let results = models
        .select(Model::as_select())
        .load(conn)
        .expect("Error loading models");
    return results;
}
//EQUIPMENT
pub fn create_equipment(conn: &mut SqliteConnection, name: &str, km: &i32,  model_id: &i32, nserial: &str, notes: &str) -> Result<Equipment, diesel::result::Error> {
    use crate::schema::equipments;

    let new_equipment = NewEquipment { name, km, model_id, nserial, notes };

    match diesel::insert_into(equipments::table)
    .values(&new_equipment)
    .returning(Equipment::as_returning())
    .get_result(conn)
{
    Ok(equipment) => {
        println!("Saved equipment {:?}", equipment);
        Ok(equipment)
    }
    Err(err) => {
        eprintln!("Error saving equipment: {:?}", err);
        Err(err)
    }
}
}

pub fn find_all_equipments(conn: &mut SqliteConnection) -> Vec<Equipment> {
    use self::schema::equipments::dsl::*;

    let results = equipments
        .select(Equipment::as_select())
        .load(conn)
        .expect("Error loading equipments");
    return results;
}
pub fn find_equipment_by_id(conn: &mut SqliteConnection, equipment_id: i32) -> Equipment {
    use self::schema::equipments::dsl::*;

    let equipment = equipments
        .find(equipment_id)
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
pub fn create_log(conn: &mut SqliteConnection, equipment_id: &i32, person_id: &i32, description: &String, km: &i32, job: &String ) -> Log {
    use crate::schema::logs;

    let new_log = NewLog { equipment_id, person_id, description, km, job };

    let log = diesel::insert_into(logs::table)
        .values(&new_log)
        .returning(Log::as_returning())
        .get_result(conn)
        .expect("Error saving new equipment");

    return log;
}
pub fn update_log_created_at_by_id(conn: &mut SqliteConnection, log_id: &i32, new_created_at: &String) -> Log {
    use self::schema::logs::dsl::*;

    let log = diesel::update(logs.find(log_id))
    .set(created_at.eq(new_created_at))
    .get_result(conn)
    .expect("Error loading equipment");
    
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
//REVISIONS
pub fn find_all_revisions(conn: &mut SqliteConnection) -> Vec<Revision> {
    use self::schema::revisions::dsl::*;
    let results = revisions
        .select(Revision::as_select())
        .load(conn)
        .expect("Error loading revisions");
    return results;
}

pub fn create_revision(conn: &mut SqliteConnection, equipment_id: &i32, tipo: &String, target: &String) -> Revision {
    use crate::schema::revisions;

    let new_revision = models::NewRevision { equipment_id, tipo, target };

    let revision = diesel::insert_into(revisions::table)
        .values(&new_revision)
        .returning(Revision::as_returning())
        .get_result(conn)
        .expect("Error saving new equipment");

    return revision;
}