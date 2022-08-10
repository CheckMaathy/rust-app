#[macro_use] extern crate rocket;

use lib::handler::hello;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}