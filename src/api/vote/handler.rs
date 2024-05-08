use rocket::{
    http::Status,
    serde::json::Json,
};
use rocket_db_pools::Connection;

use super::Vote;
use crate::{
    response::{
        error::ApiError,
        ResponseBody,
        ToStatus,
    },
    DbPool,
};

#[put("/vote", format = "application/json", data = "<vote>")]
pub(crate) async fn vote(
    vote: Json<Vote>,
    mut connection: Connection<DbPool>,
) -> (Status, Json<ResponseBody<Vote, ApiError<sqlx::Error>>>) {
    let result = super::create_or_update_vote(&vote, &mut **connection).await;

    match result {
        Ok((vote_existed, vote)) => match vote_existed {
            false => (Status::Created, Json(Ok(vote).into())),
            true => (Status::Ok, Json(Ok(vote).into())),
        },
        Err(sqlx::Error::RowNotFound) => (
            Status::UnprocessableEntity,
            Json(Err(sqlx::Error::RowNotFound.into()).into()),
        ),
        Err(error) => (error.to_status(), Json(Err(error.into()).into())),
    }
}
