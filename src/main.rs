#[macro_use]
extern crate rocket;

mod schema;

use dotenvy::dotenv;
use std::env;

#[get("/")]
fn hello() -> String {
    String::from("Hello, world!")
}

#[launch]
fn rocket() -> _ {
    dotenv().expect(".env file not found");

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    rocket::build().mount("/", routes![hello])
}
