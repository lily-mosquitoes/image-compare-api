use rocket::{
    http::Status,
    serde::{
        json::Json,
        uuid::Uuid,
    },
};
use rocket_db_pools::Connection;

use super::User;
use crate::{
    request::RequestError,
    response::Response,
    DbPool,
};

#[get("/user/<id>")]
pub(crate) async fn get_user(
    id: Uuid,
    mut connection: Connection<DbPool>,
) -> (Status, Json<Response<User, RequestError<sqlx::Error>>>) {
    let result = super::get_user(id, &mut **connection)
        .await
        .map_err(|error| error.into());
    let response = Response::from_result(result);

    (response.status(), Json(response))
}

#[post("/user")]
pub(crate) async fn generate_user(
    mut connection: Connection<DbPool>,
) -> (Status, Json<Response<User, RequestError<sqlx::Error>>>) {
    let result = super::generate_user(&mut **connection)
        .await
        .map_err(|error| error.into());
    let response =
        Response::from_result(result).with_success_status(Status::Created);

    (response.status(), Json(response))
}
