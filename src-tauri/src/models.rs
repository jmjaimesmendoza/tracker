use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::equipments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Equipment {
    pub id: i32,
    pub name: String,
    pub km: i32,
    pub model_id: i32,
    pub nserial: String,
    pub notes: String,
    pub file_path: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::brands)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Brand {
    pub id: i32,
    pub name: String,
}


#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::models)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Model {
    pub id: i32,
    pub brand_id: i32,
    pub name: String,
}


#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::logs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Log {
    pub id: i32,
    pub equipment_id: i32,
    pub person_id: i32,
    pub job: String,
    pub km: i32,
    pub description: String,
    pub created_at: String
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::revisions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Revision {
    pub id: i32,
    pub equipment_id: i32,
    pub tipo: String,
    pub target: String,
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
use crate::schema::revisions;
use crate::schema::brands;
use crate::schema::models;

#[derive(Insertable)]
#[diesel(table_name = equipments)]
pub struct NewEquipment<'a> {
    pub name: &'a str,
    pub km: &'a i32,
    pub model_id: &'a i32,
    pub nserial: &'a str,
    pub notes: &'a str,
    pub file_path: &'a str,
}
#[derive(Insertable)]
#[diesel(table_name = logs)]
pub struct NewLog<'a> {
    pub equipment_id: &'a i32,
    pub person_id: &'a i32,
    pub description: &'a String,
    pub job: &'a String,
    pub km: &'a i32,
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

#[derive(Insertable)]
#[diesel(table_name = revisions)]
pub struct NewRevision<'a> {
    pub equipment_id: &'a i32,
    pub tipo: &'a String,
    pub target: &'a String,
}
#[derive(Insertable)]
#[diesel(table_name = brands)]
pub struct NewBrand<'a> {
    pub name: &'a String,
}
#[derive(Insertable)]
#[diesel(table_name = models)]
pub struct NewModel<'a> {
    pub brand_id: &'a i32,
    pub name: &'a String,
}