#[macro_use]
extern crate rocket;

mod api;

use api::healthcheck::handler;

#[launch]
pub(crate) fn rocket() -> _ {
    rocket::build().mount("/api", routes![handler::healthcheck,])
}
