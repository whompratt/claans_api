use crate::schema::claans;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Serialize, Ord, Eq, PartialOrd, PartialEq)]
pub struct Claan {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = claans)]
pub struct NewClaan {
    pub name: String,
}
