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
    fairing,
    fs::FileServer,
    http::{
        uri::Origin,
        Header,
    },
    Build,
    Rocket,
};

pub fn rocket<P: AsRef<Path>>(static_dir: P) -> Rocket<Build> {
    let static_dir = StaticDir {
        path: static_dir.as_ref().to_path_buf(),
        origin: Origin::parse("/static/images").unwrap(),
    };

    rocket::build()
        .attach(CORS)
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

pub(crate) struct StaticDir {
    pub(crate) path: PathBuf,
    pub(crate) origin: Origin<'static>,
}

struct CORS;

#[rocket::async_trait]
impl fairing::Fairing for CORS {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "CORS Headers",
            kind: fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://127.0.0.1:9000",
        ));
    }
}
