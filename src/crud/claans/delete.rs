use crate::database::establish_connection;
use crate::models::claans::Claan;
use crate::models::response_models::{Response, ResponseBody};
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn delete_claan(claan_id: i32) -> Result<Vec<Claan>, NotFound<String>> {
    use crate::database::schema::claans;
    use crate::database::schema::claans::dsl::*;

    let response: Response;

    let num_deleted =
        match diesel::delete(claans.filter(id.eq(claan_id))).execute(&mut establish_connection()) {
            Ok(count) => count,
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    let response = Response {
                        body: ResponseBody::Message(format!(
                            "Error deleting post with id {} - {}",
                            claan_id, err
                        )),
                    };
                    return Err(NotFound(serde_json::to_string(&response).unwrap()));
                }
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        };

    if num_deleted > 0 {
        match claans::table
            .select(claans::all_columns)
            .load::<Claan>(&mut establish_connection())
        {
            Ok(mut claans_) => {
                claans_.sort();
                Ok(claans_)
            }
            Err(err) => panic!("Database error - {}", err),
        }
    } else {
        response = Response {
            body: ResponseBody::Message(format!("Error - no claans with id {}", claan_id)),
        };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}
