use crate::database::establish_connection;
use crate::models::claans::Claan;
use crate::models::response_models::{Response, ResponseBody};
use diesel::prelude::*;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

pub fn update_claan(claan: Json<Claan>) -> Result<Claan, NotFound<String>> {
    // TODO: Add filter so update isn't applying to ALL rows
    use crate::database::schema::claans::dsl::*;

    match diesel::update(claans.find(claan.id))
        .set(claan.into_inner())
        .returning(Claan::as_returning())
        .get_result::<Claan>(&mut establish_connection())
    {
        Ok(claan_) => Ok(claan_),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!("Error updating claan - {}", err)),
                };
                Err(NotFound(serde_json::to_string(&response).unwrap()))
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
