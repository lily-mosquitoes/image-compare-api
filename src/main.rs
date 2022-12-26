mod api;
mod logger;
mod response;

#[macro_use]
extern crate rocket;

use dotenvy_macro::dotenv;
use rocket::{
    fs::FileServer,
    serde::json::Json,
};
use serde::Serialize;

use crate::{
    api::{
        healthcheck::handler::healthcheck,
        images::handler::images_to_compare,
    },
    response::Response,
};

pub(crate) static STATIC_FILES_DIR: &'static str =
    dotenv!("STATIC_FILES_DIR");

#[derive(Debug, Serialize)]
enum NotFound {
    ResourceNotFound(String),
}

#[catch(404)]
async fn not_found(
    request: &rocket::Request<'_>,
) -> Json<Response<(), NotFound>> {
    let traceback =
        Some(NotFound::ResourceNotFound(request.uri().to_string()));

    Json(Response::build().set_traceback(traceback))
}

#[launch]
pub(crate) fn rocket() -> _ {
    let logger = crate::logger::setup();
    if let Err(error) = logger {
        // error happens on tests when rocket is initialized multiple
        // times
        error!("{}", error);
    }

    rocket::build()
        .register("/", catchers![not_found])
        .mount("/api", routes![healthcheck, images_to_compare])
        .mount("/images", FileServer::from(STATIC_FILES_DIR))
}
