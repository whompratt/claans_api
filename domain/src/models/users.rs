use diesel::prelude::*;
use infrastructure::schema::users;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Serialize, Ord, Eq, PartialOrd, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub claan_id: i32,
    pub active: bool,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub claan_id: i32,
    pub active: bool,
}
