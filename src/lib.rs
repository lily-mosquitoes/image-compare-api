mod api;
mod catchers;
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
use rocket_db_pools::Database;
use sqlx::{
    sqlite::SqliteConnectOptions,
    ConnectOptions,
};

pub fn rocket<P: AsRef<Path>>(
    static_dir: P,
    connection_options: SqliteConnectOptions,
) -> Rocket<Build> {
    let static_dir = StaticDir {
        path: static_dir.as_ref().to_path_buf(),
        origin: Origin::parse("/static/images").unwrap(),
    };

    let figment = rocket::Config::figment().merge((
        "databases.main",
        rocket_db_pools::Config {
            url: connection_options.to_url_lossy().to_string(),
            min_connections: None,
            max_connections: 10,
            connect_timeout: 3,
            idle_timeout: None,
        },
    ));

    rocket::custom(figment)
        .attach(CORS)
        .attach(DbPool::init())
        .register("/", catchers![crate::catchers::not_found])
        .mount(
            "/api",
            routes![
                crate::api::healthcheck::handler::healthcheck,
                crate::api::comparison::handler::get_comparison_for_user,
                crate::api::user::handler::get_user,
                crate::api::user::handler::generate_user,
                crate::api::vote::handler::vote,
            ],
        )
        .mount(static_dir.origin.clone(), FileServer::from(&static_dir.path))
        .manage(static_dir)
}

pub(crate) struct StaticDir {
    pub(crate) path: PathBuf,
    pub(crate) origin: Origin<'static>,
}

#[derive(Database)]
#[database("main")]
pub(crate) struct DbPool(sqlx::SqlitePool);

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
