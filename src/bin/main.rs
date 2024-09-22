#[macro_use]
extern crate rocket;
use api::handlers::claans;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![
            claans::get_claans,
            claans::get_claan,
            claans::update_claan,
            claans::create_claan,
            claans::delete_claan,
        ],
    )
}
