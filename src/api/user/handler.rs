use rocket::{
    http::Status,
    serde::{
        json::Json,
        uuid::Uuid,
    },
};

use super::User;
use crate::{
    request::RequestError,
    response::Response,
};

#[get("/user/<id>")]
pub(crate) async fn get_user(
    id: Uuid,
) -> (Status, Json<Response<User, RequestError<sqlx::Error>>>) {
    let result = super::get_user(id).map_err(|error| error.into());
    let response = Response::from_result(result);

    (response.status(), Json(response))
}

#[post("/user")]
pub(crate) async fn generate_user(
) -> (Status, Json<Response<User, RequestError<sqlx::Error>>>) {
    let result = super::generate_user().map_err(|error| error.into());
    let response = Response::from_result(result);

    (response.status(), Json(response))
}
