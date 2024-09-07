use diesel::prelude::*;
use domain::models::claans::{Claan, NewClaan};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

pub fn create_claan(claan: Json<NewClaan>) -> Created<String> {
    use infrastructure::schema::claans;

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
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
