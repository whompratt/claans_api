use diesel::prelude::*;
use domain::models::claans::Claan;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn delete_claan(claan_id: i32) -> Result<Vec<Claan>, NotFound<String>> {
    use infrastructure::schema::claans;
    use infrastructure::schema::claans::dsl::*;

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
