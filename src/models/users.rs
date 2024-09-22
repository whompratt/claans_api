use crate::crud::claans::read;
use crate::database::schema::users;
use crate::models::claans::Claan;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::fmt;

#[derive(
    Debug, Serialize, Ord, Eq, PartialOrd, PartialEq, Queryable, Identifiable, AsChangeset,
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
        write!(
            f,
            "<User {name}, {email}",
            name = self.name,
            email = self.email
        )
    }
}

impl User {
    pub fn get_claan(&self) -> Claan {
        let claan = read::list_claan(self.claan_id);
        todo!();
    }
}
