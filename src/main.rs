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

mod api;

use api::healthcheck::handler;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Response<T> {
    timestamp: DateTime<Utc>,
    // request_id: RequestId,
    // traceback: Option<Error>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

#[launch]
pub(crate) fn rocket() -> _ {
    rocket::build().mount("/api", routes![handler::healthcheck,])
}
