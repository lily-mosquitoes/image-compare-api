use rocket::{
    http::Status,
    serde::json::Json,
};

use crate::{
    request::RequestError,
    response::{
        Response,
        ToStatus,
    },
};

#[derive(Debug)]
pub(crate) struct NotFound;

impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resource not found")
    }
}

impl std::error::Error for NotFound {}

impl ToStatus for NotFound {
    fn to_status(&self) -> Status {
        Status::NotFound
    }
}

#[catch(404)]
pub(crate) async fn not_found() -> Json<Response<(), RequestError<NotFound>>> {
    let result = Err(NotFound.into());

    Json(Response::from_result(result))
}
