mod api;

#[macro_use]
extern crate rocket;

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::api::{
    healthcheck::handler::healthcheck,
    images::handler::images_to_compare,
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Response<T, E> {
    timestamp: DateTime<Utc>,
    // request_id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    traceback: Option<E>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            if !record.target().contains("rocket") {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now()
                        .format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message,
                ))
            }
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
        .mount("/api", routes![healthcheck, images_to_compare])
}
