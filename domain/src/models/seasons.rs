use chrono::NaiveDate;
use diesel::prelude::*;
use infrastructure::schema::seasons;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Serialize, Ord, Eq, PartialOrd, PartialEq)]
pub struct Season {
    pub id: i32,
    pub name: String,
    pub start_date: NaiveDate,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = seasons)]
pub struct NewSeason {
    pub name: String,
    pub start_date: NaiveDate,
}
