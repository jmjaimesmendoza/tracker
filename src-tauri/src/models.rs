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

use crate::schema::equipments;

#[derive(Insertable)]
#[diesel(table_name = equipments)]
pub struct NewEquipments<'a> {
    pub name: &'a str,
    pub km: &'a i32,
}