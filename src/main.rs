mod api;
mod logger;
mod response;
mod statics;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

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
    if let Err(error) = crate::logger::setup() {
        error!("{}", error);
        panic!("failed to setup logger");
    }

    rocket::build()
        .register("/", catchers![not_found])
        .mount("/api", routes![healthcheck, images_to_compare])
        .mount(
            "/images",
            FileServer::from(&*crate::statics::STATIC_FILES_DIR),
        )
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
        let static_files_dir = &*crate::statics::STATIC_FILES_DIR;

        let entries: Vec<OsString> = fs::read_dir(static_files_dir)
            .expect("`STATIC_FILES_DIR` to exist and be accessible")
            .filter_map(|x| x.ok())
            .map(|x| x.file_name())
            .collect();

        entries.contains(&OsString::from(file_name))
    }
}
