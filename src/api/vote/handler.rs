use std::net::IpAddr;

use rocket::{
    http::Status,
    serde::json::Json,
};
use rocket_db_pools::Connection;

use super::Vote;
use crate::{
    api::{
        QueryError,
        RequestId,
    },
    response::ResponseBody,
    DbPool,
};

#[put("/vote", format = "application/json", data = "<vote>")]
pub(crate) async fn vote(
    mut vote: Json<Vote>,
    ip_addr: Option<IpAddr>,
    request_id: &RequestId,
    mut connection: Connection<DbPool>,
) -> (Status, Json<ResponseBody<Vote, QueryError>>) {
    vote.ip_addr = ip_addr.map(|ip| ip.to_canonical().to_string());
    let result = super::create_or_update_vote(&vote, &mut **connection).await;

    match result {
        Err(QueryError::RowNotFound(message)) => (
            Status::UnprocessableEntity,
            Json((request_id, Err(QueryError::RowNotFound(message))).into()),
        ),
        Err(error) => {
            (error.default_status(), Json((request_id, Err(error)).into()))
        },
        Ok(vote) => (vote.status(), Json((request_id, Ok(vote)).into())),
    }
}
