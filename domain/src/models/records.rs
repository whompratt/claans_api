use crate::schema::records;
use chrono::NaiveDate;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Serialize, Ord, Eq, PartialOrd, PartialEq)]
pub struct Record {
    pub id: i32,
    pub score: i32,
    pub timestamp: NaiveDate,
    pub task_id: i32,
    pub user_id: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = records)]
pub struct NewRecord {
    pub score: i32,
    pub timestamp: NaiveDate,
    pub task_id: i32,
    pub user_id: i32,
}
