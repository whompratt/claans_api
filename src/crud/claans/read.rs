use crate::database::establish_connection;
use crate::models::claans::Claan;
use crate::models::response_models::{Response, ResponseBody};
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_claan(claan_id: i32) -> Result<Claan, NotFound<String>> {
    use crate::database::schema::claans;

    match claans::table
        .find(claan_id)
        .first::<Claan>(&mut establish_connection())
    {
        Ok(claan) => Ok(claan),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error selecting claan with id {} - {}",
                        claan_id, err
                    )),
                };
                Err(NotFound(serde_json::to_string(&response).unwrap()))
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_claans() -> Vec<Claan> {
    use crate::database::schema::claans;

    match claans::table
        .select(claans::all_columns)
        .load::<Claan>(&mut establish_connection())
    {
        Ok(mut claans) => {
            claans.sort();
            claans
        }
        Err(err) => {
            panic!("Database error - {}", err);
        }
    }
}
