use rocket::{
    http::Status,
    serde::json::Json,
};

use crate::response::ResponseBody;

#[get("/healthcheck")]
pub(crate) async fn healthcheck() -> (Status, Json<ResponseBody<(), ()>>) {
    (Status::Ok, Json(Ok(()).into()))
}
