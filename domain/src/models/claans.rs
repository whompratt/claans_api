use diesel::prelude::*;
use infrastructure::schema::claans;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(
    Queryable,
    Serialize,
    Ord,
    Eq,
    PartialOrd,
    PartialEq,
    AsChangeset,
    Selectable,
    serde::Deserialize,
)]
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
