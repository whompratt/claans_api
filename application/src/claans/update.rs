use diesel::prelude::*;
use domain::models::claans::Claan;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

pub fn update_claan(claan: Json<Claan>) -> Result<Claan, NotFound<String>> {
    use infrastructure::schema::claans;

    match diesel::update(claans::table)
        .set(claan.into_inner())
        .returning(Claan::as_returning())
        .get_result::<Claan>(&mut establish_connection())
    {
        Ok(claan_) => claan_,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!("Error updating claan - {}", err)),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    todo!();
}
