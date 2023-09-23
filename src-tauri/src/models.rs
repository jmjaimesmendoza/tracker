use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::equipments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Equipment {
    pub id: i32,
    pub name: String,
    pub km: i32,
}
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::logs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Log {
    pub id: i32,
    pub equipment_id: i32,
    pub person_id: i32,
    pub previous_km: i32,
    pub new_km: i32
}
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::persons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Person {
    pub id: i32,
    pub name: String
}
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hash: String
}

use crate::schema::equipments;
use crate::schema::logs;
use crate::schema::persons;
use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = equipments)]
pub struct NewEquipment<'a> {
    pub name: &'a str,
    pub km: &'a i32,
}
#[derive(Insertable)]
#[diesel(table_name = logs)]
pub struct NewLog<'a> {
    pub equipment_id: &'a i32,
    pub person_id: &'a i32,
    pub description: &'a String,
    pub previous_km: &'a i32,
    pub new_km: &'a i32,
}

#[derive(Insertable)]
#[diesel(table_name = persons)]
pub struct NewPerson<'a> {
    pub name: &'a str,
}
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hash: &'a str,
}