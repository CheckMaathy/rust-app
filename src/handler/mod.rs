// Handlers

use rocket::http::RawStr;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
pub fn userByName(name: String) -> String {
    format!("Hello, {name}")
}