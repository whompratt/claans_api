use crate::database::establish_connection;
use crate::models::response_models::{Response, ResponseBody};
use crate::models::users::{NewUser, User};
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_user(user: Json<NewUser>) -> Created<String> {
    use crate::database::schema::users;

    let user = user.into_inner();

    match diesel::insert_into(users::table)
        .values(&user)
        .get_result::<User>(&mut establish_connection())
    {
        Ok(user) => {
            let response = Response {
                body: ResponseBody::User(user),
            };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => {
            panic!("Database error - {}", err);
        }
    }
}
