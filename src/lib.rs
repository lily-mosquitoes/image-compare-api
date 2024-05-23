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
        Header,
        Method,
    },
    Build,
    Rocket,
};
use rocket_db_pools::Database;
use sqlx::{
    sqlite::SqliteConnectOptions,
    ConnectOptions,
};

pub fn rocket<S: Into<String>, P: AsRef<Path>>(
    allowed_origin: S,
    static_dir: P,
    connection_options: SqliteConnectOptions,
) -> Rocket<Build> {
    let allowed_origin = allowed_origin.into();

    let static_dir = StaticDir {
        path: static_dir.as_ref().to_path_buf(),
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
        .attach(CORS { allowed_origin })
        .attach(DbPool::init())
        .attach(DbMigrations)
        .register(
            "/",
            catchers![
                crate::catchers::default,
                crate::catchers::unprocessable_entity,
                crate::catchers::not_found,
                crate::catchers::unauthorized,
            ],
        )
        .mount(
            "/api",
            routes![
                crate::api::options::options,
                crate::api::healthcheck::handler::healthcheck,
                crate::api::comparison::handler::get_comparison_for_user,
                crate::api::user::handler::get_user,
                crate::api::user::handler::generate_user,
                crate::api::vote::handler::vote,
                crate::api::admin::handler::generate_comparisons,
            ],
        )
        .mount(STATIC_ROUTE, FileServer::from(&static_dir.path))
        .manage(static_dir)
}

static STATIC_ROUTE: &'static str = "/static/images";

pub(crate) struct StaticDir {
    pub(crate) path: PathBuf,
}

#[derive(Database)]
#[database("main")]
pub(crate) struct DbPool(sqlx::SqlitePool);

struct DbMigrations;

#[rocket::async_trait]
impl fairing::Fairing for DbMigrations {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "SQLX Migrations",
            kind: fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        if let Some(connection) = DbPool::fetch(&rocket) {
            match sqlx::migrate!().run(&**connection).await {
                Ok(_) => Ok(rocket),
                Err(error) => {
                    error!("Migrations failed: {error}");
                    Err(rocket)
                },
            }
        } else {
            Err(rocket)
        }
    }
}

struct CORS {
    allowed_origin: String,
}

#[rocket::async_trait]
impl fairing::Fairing for CORS {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "CORS Headers",
            kind: fairing::Kind::Response | fairing::Kind::Request,
        }
    }

    async fn on_response<'r>(
        &self,
        request: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        if request.method() == Method::Options {
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "OPTIONS, POST, DELETE, GET",
            ));
            response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                "Content-Type, Authorization",
            ));
        }

        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            self.allowed_origin.clone(),
        ));
    }

    async fn on_request(
        &self,
        request: &mut rocket::Request<'_>,
        _: &mut rocket::Data<'_>,
    ) {
        if request.method() == Method::Options {
            request.set_uri(uri!("/api/options"))
        }
    }
}
