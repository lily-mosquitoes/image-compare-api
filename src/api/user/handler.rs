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
    response::{
        error::ApiError,
        ResponseBody,
        ToStatus,
    },
    DbPool,
};

#[get("/user/<id>")]
pub(crate) async fn get_user(
    id: Uuid,
    mut connection: Connection<DbPool>,
) -> (Status, Json<ResponseBody<User, ApiError<sqlx::Error>>>) {
    let result: ResponseBody<_, _> = super::get_user(id, &mut **connection)
        .await
        .map_err(|error| error.into())
        .into();

    (result.status(), Json(result))
}

#[post("/user")]
pub(crate) async fn generate_user(
    mut connection: Connection<DbPool>,
) -> (Status, Json<ResponseBody<User, ApiError<sqlx::Error>>>) {
    let result = super::generate_user(&mut **connection).await;

    match result {
        Err(error) => (error.to_status(), Json(Err(error.into()).into())),
        Ok(user) => (Status::Created, Json(Ok(user).into())),
    }
}
