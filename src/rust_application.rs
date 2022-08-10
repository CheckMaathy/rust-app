#[macro_use] extern crate rocket;

use lib::handler::hello;
use lib::handler::userByName;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, userByName])
}