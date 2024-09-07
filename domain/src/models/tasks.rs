use crate::models::dice::Dice;
use crate::models::tasktype::Tasktype;
use crate::schema::tasks;
use chrono::NaiveDate;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Serialize, Ord, Eq, PartialOrd, PartialEq)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub tasktype: Tasktype,
    pub dice: Dice,
    pub ephemeral: bool,
    pub active: bool,
    pub last: NaiveDate,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub description: String,
    pub tasktype: Tasktype,
    pub dice: Dice,
    pub ephemeral: bool,
    pub active: bool,
    pub last: NaiveDate,
}
