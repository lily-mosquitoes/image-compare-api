use rocket::serde::json::Json;
use serde::Serialize;

use crate::response::Response;

#[derive(Debug, Serialize)]
pub(crate) enum NotFound {
    ResourceNotFound(String),
}

#[catch(404)]
pub(crate) async fn not_found(
    request: &rocket::Request<'_>,
) -> Json<Response<(), NotFound>> {
    let result =
        Err(NotFound::ResourceNotFound(request.uri().to_string()));

    Json(Response::from_result(result))
}
