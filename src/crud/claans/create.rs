use crate::database::establish_connection;
use crate::models::claans::{Claan, NewClaan};
use crate::models::response_models::{Response, ResponseBody};
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_claan(claan: Json<NewClaan>) -> Created<String> {
    use crate::database::schema::claans;

    let claan = claan.into_inner();

    match diesel::insert_into(claans::table)
        .values(&claan)
        .get_result::<Claan>(&mut establish_connection())
    {
        Ok(claan) => {
            let response = Response {
                body: ResponseBody::Claan(claan),
            };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => {
            panic!("Database error - {}", err);
        }
    }
}
