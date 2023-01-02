use rocket::{
    http::Status,
    serde::json::Json,
};

use crate::response::Response;

#[get("/healthcheck")]
pub(crate) async fn healthcheck() -> (Status, Json<Response<(), ()>>)
{
    let response = Response::from_result(Ok(()));

    (Status::Ok, Json(response))
}
