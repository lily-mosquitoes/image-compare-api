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
pub(crate) async fn comparison(
    id: Uuid,
) -> (Status, Json<Response<User, RequestError>>) {
    let result = super::get_user(id);
    let response = Response::from_result(result);

    (response.status(), Json(response))
}
