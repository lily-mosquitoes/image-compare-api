use rocket::{
    http::Status,
    serde::json::Json,
};

use crate::{
    api::RequestId,
    response::ResponseBody,
};

#[get("/healthcheck")]
pub(crate) async fn healthcheck(
    request_id: &RequestId,
) -> (Status, Json<ResponseBody<(), ()>>) {
    (Status::Ok, Json((request_id, Ok(())).into()))
}
