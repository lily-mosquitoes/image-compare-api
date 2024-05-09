use rocket::{
    http::Status,
    serde::json::Json,
};
use rocket_db_pools::Connection;

use super::Vote;
use crate::{
    api::QueryError,
    response::ResponseBody,
    DbPool,
};

#[put("/vote", format = "application/json", data = "<vote>")]
pub(crate) async fn vote(
    vote: Json<Vote>,
    mut connection: Connection<DbPool>,
) -> (Status, Json<ResponseBody<Vote, QueryError>>) {
    let result = super::create_or_update_vote(&vote, &mut **connection).await;

    match result {
        Err(QueryError::RowNotFound(message)) => (
            Status::UnprocessableEntity,
            Json(Err(QueryError::RowNotFound(message)).into()),
        ),
        Err(error) => (error.default_status(), Json(Err(error).into())),
        Ok((status, vote)) => (status, Json(Ok(vote).into())),
    }
}
