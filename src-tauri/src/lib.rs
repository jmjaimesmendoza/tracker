pub mod models;
pub mod schema;

use crate::models::{Brand, Model, NewBrand, NewModel};

use self::models::{Equipment, NewEquipment};
use self::models::{Log, NewLog};
use self::models::{NewPerson, Person};
use self::models::{NewManual, Manual};
use diesel::prelude::*;
use models::Revision;

pub fn establish_connection() -> SqliteConnection {
    let database_url = "./tracker.sqlite";
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

pub fn create_manual(conn: &mut SqliteConnection, name: &String, file_path: &String, equipment_id: &i32) -> Manual {
    use crate::schema::manuals;

    let new_manual = NewManual { name, file_path, equipment_id };

    let manual = diesel::insert_into(manuals::table)
        .values(&new_manual)
        .returning(Manual::as_returning())
        .get_result(conn)
        .expect("Error saving new manual");

    return manual;
}

pub fn find_all_manuals(conn: &mut SqliteConnection) -> Vec<Manual> {
    use self::schema::manuals::dsl::*;
    let results = manuals
        .select(Manual::as_select())
        .load(conn)
        .expect("Error loading manuals");
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

pub fn update_model(
    conn: &mut SqliteConnection,
    model_id: &i32,
    new_name: &String,
    new_brand_id: &i32,
) -> Model {
    use self::schema::models::dsl::*;

    let model = diesel::update(models.find(model_id))
        .set((name.eq(new_name), brand_id.eq(new_brand_id)))
        .get_result(conn)
        .expect("Error loading model");

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
pub fn create_equipment(
    conn: &mut SqliteConnection,
    name: &str,
    km: &i32,
    model_id: &i32,
    nserial: &str,
    notes: &str,
    file_path: &str,
) -> Equipment {
    use crate::schema::equipments;

    let new_equipment = NewEquipment {
        name,
        km,
        model_id,
        nserial,
        notes,
        file_path,
    };

    let equipment = diesel::insert_into(equipments::table)
        .values(&new_equipment)
        .returning(Equipment::as_returning())
        .get_result(conn)
        .expect("Error saving new equipment");

    return equipment;
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
    return equipment;
}
pub fn update_equipment_km_by_id(
    conn: &mut SqliteConnection,
    equipment_id: &i32,
    new_km: &i32,
) -> Equipment {
    use self::schema::equipments::dsl::*;

    let equipment = diesel::update(equipments.find(equipment_id))
        .set(km.eq(new_km))
        .get_result(conn)
        .expect("Error loading equipment");
    return equipment;
}
pub fn update_equipment(
    conn: &mut SqliteConnection,
    equipment_id: &i32,
    new_name: &str,
    new_km: &i32,
    new_model_id: &i32,
    new_nserial: &str,
    new_notes: &str,
    new_file_path: &str,
) -> Equipment {
    use self::schema::equipments::dsl::*;

    let equipment = diesel::update(equipments.find(equipment_id))
        .set((
            name.eq(new_name),
            km.eq(new_km),
            model_id.eq(new_model_id),
            nserial.eq(new_nserial),
            notes.eq(new_notes),
            file_path.eq(new_file_path),
        ))
        .get_result(conn)
        .expect("Error loading equipment");
    return equipment;
}
//LOGS
pub fn create_log(
    conn: &mut SqliteConnection,
    equipment_id: &i32,
    person_id: &i32,
    description: &String,
    km: &i32,
    job: &String,
) -> Log {
    use crate::schema::logs;

    let new_log = NewLog {
        equipment_id,
        person_id,
        description,
        km,
        job,
    };

    let log = diesel::insert_into(logs::table)
        .values(&new_log)
        .returning(Log::as_returning())
        .get_result(conn)
        .expect("Error saving new equipment");

    return log;
}
pub fn update_log(
    conn: &mut SqliteConnection,
    log_id: &i32,
    new_equipment_id: &i32,
    new_person_id: &i32,
    new_description: &String,
    new_km: &i32,
    new_job: &String,
    new_created_at: &String,
) -> Log {
    use self::schema::logs::dsl::*;

    let log = diesel::update(logs.find(log_id))
        .set((
            equipment_id.eq(new_equipment_id),
            person_id.eq(new_person_id),
            description.eq(new_description),
            km.eq(new_km),
            job.eq(new_job),
            created_at.eq(new_created_at),
        ))
        .get_result(conn)
        .expect("Error loading equipment");

    return log;
}

pub fn update_log_created_at_by_id(
    conn: &mut SqliteConnection,
    log_id: &i32,
    new_created_at: &String,
) -> Log {
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

pub fn create_revision(
    conn: &mut SqliteConnection,
    equipment_id: &i32,
    tipo: &String,
    target: &String,
) -> Revision {
    use crate::schema::revisions;

    let new_revision = models::NewRevision {
        equipment_id,
        tipo,
        target,
    };

    let revision = diesel::insert_into(revisions::table)
        .values(&new_revision)
        .returning(Revision::as_returning())
        .get_result(conn)
        .expect("Error saving new equipment");

    return revision;
}
