mod api;
mod catchers;
mod request;
mod response;

#[macro_use]
extern crate rocket;

use std::path::{
    Path,
    PathBuf,
};

use rocket::{
    fs::FileServer,
    http::uri::Origin,
    Build,
    Rocket,
};

pub(crate) struct StaticDir {
    pub(crate) path: PathBuf,
    pub(crate) origin: Origin<'static>,
}

pub fn rocket<P: AsRef<Path>>(static_dir: P) -> Rocket<Build> {
    let static_dir = StaticDir {
        path: static_dir.as_ref().to_path_buf(),
        origin: Origin::parse("/static/images").unwrap(),
    };

    rocket::build()
        .register("/", catchers![crate::catchers::not_found])
        .mount(
            "/api",
            routes![
                crate::api::healthcheck::handler::healthcheck,
                crate::api::comparison::handler::comparison,
                crate::api::user::handler::user,
            ],
        )
        .mount(static_dir.origin.clone(), FileServer::from(&static_dir.path))
        .manage(static_dir)
}
