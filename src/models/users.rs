use crate::crud::claans::read;
use crate::database::schema::users;
use crate::models::claans::Claan;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::fmt;

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Ord, Eq, PartialOrd, PartialEq, AsChangeset,
)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub claan_id: i32,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub active: bool,
    pub current_auth_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_action: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub claan_id: i32,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub active: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let claan_name: String = match self.get_claan() {
            Some(claan) => claan.name,
            None => String::from("no claan"),
        };

        write!(
            f,
            "<User {name}, {email}, in {claan_name}",
            name = self.name,
            email = self.email,
            claan_name = claan_name,
        )
    }
}

impl User {
    pub fn get_claan(&self) -> Option<Claan> {
        match read::list_claan(self.claan_id) {
            Ok(claan) => Some(claan),
            Err(_) => None,
        }
    }
}
