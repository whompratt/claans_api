use application::claans::{create, delete, read, update};
use domain::models::claans::{Claan, NewClaan};
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post};
use shared::response_models::{Response, ResponseBody};

#[get("/claans")]
pub fn get_claans() -> String {
    let claans: Vec<Claan> = read::list_claans();
    let response = Response {
        body: ResponseBody::Claans(claans),
    };

    serde_json::to_string(&response).unwrap()
}

#[get("/claans/<claan_id>")]
pub fn get_claan(claan_id: i32) -> Result<String, NotFound<String>> {
    let claan: Claan = read::list_claan(claan_id)?;
    let response = Response {
        body: ResponseBody::Claan(claan),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/claans", format = "application/json", data = "<claan>")]
pub fn create_claan(claan: Json<NewClaan>) -> Created<String> {
    create::create_claan(claan)
}

#[patch("/claans", format = "application/json", data = "<claan>")]
pub fn update_claan(claan: Json<Claan>) -> Result<String, NotFound<String>> {
    let claan = update::update_claan(claan)?;
    let response = Response {
        body: ResponseBody::Claan(claan),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/claans/<claan_id>")]
pub fn delete_claan(claan_id: i32) -> Result<String, NotFound<String>> {
    let claan = delete::delete_claan(claan_id)?;
    let response = Response {
        body: ResponseBody::Claans(claan),
    };

    Ok(serde_json::to_string(&response).unwrap())
}
