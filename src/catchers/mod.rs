use rocket::serde::json::Json;

use crate::{
    request::RequestError,
    response::Response,
};

struct NotFound;

impl ToStatus

#[catch(404)]
pub(crate) async fn not_found() -> Json<Response<(), RequestError>> {
    let result = Err(RequestError::ResourceNotFound);

    Json(Response::from_result(result))
}
