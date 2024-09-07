#[macro_use]
extern crate rocket;
use api::handlers;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![
            handlers::claans::get_claan,
            handlers::claans::get_claans,
            handlers::claans::create_claan,
        ],
    )
}
