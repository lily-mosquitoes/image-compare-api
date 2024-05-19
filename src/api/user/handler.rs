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
    api::{
        QueryError,
        RequestId,
    },
    response::ResponseBody,
    DbPool,
};

#[get("/user/<id>")]
pub(crate) async fn get_user(
    id: Uuid,
    request_id: &RequestId,
    mut connection: Connection<DbPool>,
) -> (Status, Json<ResponseBody<User, QueryError>>) {
    match super::get_user(id, &mut **connection).await {
        Err(error) => {
            (error.default_status(), Json((request_id, Err(error)).into()))
        },
        Ok(user) => (Status::Ok, Json((request_id, Ok(user)).into())),
    }
}

#[post("/user")]
pub(crate) async fn generate_user(
    request_id: &RequestId,
    mut connection: Connection<DbPool>,
) -> (Status, Json<ResponseBody<User, QueryError>>) {
    match super::generate_user(&mut **connection).await {
        Err(error) => {
            (error.default_status(), Json((request_id, Err(error)).into()))
        },
        Ok(user) => (Status::Created, Json((request_id, Ok(user)).into())),
    }
}
