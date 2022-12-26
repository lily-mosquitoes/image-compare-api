mod api;

#[macro_use]
extern crate rocket;

use chrono::{
    DateTime,
    Utc,
};
use dotenvy_macro::dotenv;
use rocket::{
    fs::FileServer,
    serde::json::Json,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::api::{
    healthcheck::handler::healthcheck,
    images::handler::images_to_compare,
};

pub(crate) static STATIC_FILES_DIR: &'static str =
    dotenv!("STATIC_FILES_DIR");

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Response<T, E> {
    timestamp: DateTime<Utc>,
    // request_id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traceback: Option<E>,
}

impl<T, E> Response<T, E> {
    fn build() -> Self {
        Response::<T, E> {
            timestamp: Utc::now(),
            data: None,
            traceback: None,
        }
    }

    fn set_data(mut self, data: Option<T>) -> Self {
        self.data = data;
        self
    }

    fn set_traceback(mut self, traceback: Option<E>) -> Self {
        self.traceback = traceback;
        self
    }
}

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

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message,
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

#[launch]
pub(crate) fn rocket() -> _ {
    let logger = setup_logger();
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
