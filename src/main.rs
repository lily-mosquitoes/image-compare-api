mod api;
mod logger;
mod response;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

use std::sync::Once;

use dotenvy;
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

static LOAD_ENV: Once = Once::new();

fn load_static_files_dir() -> String {
    LOAD_ENV.call_once(|| {
        dotenvy::dotenv().expect(".env to be present");
    });

    std::env::var("STATIC_FILES_DIR")
        .expect("`STATIC_FILES_DIR to be set in .env`")
}

lazy_static! {
    pub(crate) static ref STATIC_FILES_DIR: String =
        load_static_files_dir();
}

#[derive(Debug, Serialize)]
enum NotFound {
    ResourceNotFound(String),
}

#[catch(404)]
async fn not_found(
    request: &rocket::Request<'_>,
) -> Json<Response<(), NotFound>> {
    let result =
        Err(NotFound::ResourceNotFound(request.uri().to_string()));

    Json(Response::from_result(result))
}

#[launch]
pub(crate) fn rocket() -> _ {
    #[cfg(not(test))]
    crate::logger::setup().expect("logger to start up");

    rocket::build()
        .register("/", catchers![not_found])
        .mount("/api", routes![healthcheck, images_to_compare])
        .mount("/images", FileServer::from(&*STATIC_FILES_DIR))
}

#[cfg(test)]
pub(crate) mod test_helpers {
    use std::{
        ffi::OsString,
        fs,
    };

    use rocket::local::blocking::Client;

    pub(crate) fn get_rocket_client() -> Client {
        Client::tracked(crate::rocket())
            .expect("valid rocket instance")
    }

    pub(crate) fn file_exists(file_name: &str) -> bool {
        let static_files_dir = &*crate::STATIC_FILES_DIR;

        let entries: Vec<OsString> = fs::read_dir(static_files_dir)
            .expect("`STATIC_FILES_DIR` to exist and be accessible")
            .filter_map(|x| x.ok())
            .map(|x| x.file_name())
            .collect();

        entries.contains(&OsString::from(file_name))
    }
}
