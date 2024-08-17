#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct Options<'r> {
    name: Option<&'r str>,
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/?<opt..>")]
fn hello(opt: Options<'_>) -> String {
    let mut greeting = String::new();

    greeting.push_str("Hello, ");

    if let Some(name) = opt.name {
        greeting.push_str(name);
    } else {
        greeting.push_str("world");
    }

    greeting.push('!');
    greeting
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/hello", routes![world])
}