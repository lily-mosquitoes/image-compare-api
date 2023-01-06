mod api;
mod catchers;
mod config;
mod response;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

use rocket::{
    fs::FileServer,
    Build,
    Rocket,
};

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .register("/", catchers![crate::catchers::not_found])
        .mount(
            "/api",
            routes![
                crate::api::healthcheck::handler::healthcheck,
                crate::api::images::handler::images_to_compare
            ],
        )
        .mount(
            "/api/images",
            FileServer::from(&*crate::config::STATIC_FILES_DIR),
        )
}

#[cfg(test)]
pub(crate) mod test_helpers {
    use std::{
        ffi::OsString,
        fs,
    };

    pub(crate) fn file_exists(file_name: &str) -> bool {
        let static_files_dir = &*crate::config::STATIC_FILES_DIR;

        let entries: Vec<OsString> = fs::read_dir(static_files_dir)
            .expect("`STATIC_FILES_DIR` to exist and be accessible")
            .filter_map(|x| x.ok())
            .map(|x| x.file_name())
            .collect();

        entries.contains(&OsString::from(file_name))
    }
}
