use rocket::{
    http::Status,
    serde::json::Json,
};
use rocket_db_pools::Connection;
use uuid::Uuid;

use super::Comparison;
use crate::{
    api::{
        QueryError,
        RequestId,
    },
    response::ResponseBody,
    DbPool,
};

#[get("/user/<id>/comparison")]
pub(crate) async fn get_comparison_for_user<'r>(
    id: Uuid,
    request_id: &RequestId,
    mut connection: Connection<DbPool>,
) -> (Status, Json<ResponseBody<Comparison<'r>, QueryError>>) {
    let user = crate::api::user::get_user(id, &mut **connection).await;
    let comparison =
        super::get_comparison_for_user(id, &mut **connection).await;

    match (user, comparison) {
        (Err(error), _) => {
            (error.default_status(), Json((request_id, Err(error)).into()))
        },
        (Ok(_), Err(QueryError::RowNotFound(message))) => (
            Status::ServiceUnavailable,
            Json((request_id, Err(QueryError::RowNotFound(message))).into()),
        ),
        (Ok(_), Err(error)) => {
            (error.default_status(), Json((request_id, Err(error)).into()))
        },
        (Ok(_), Ok(comparison)) => {
            (Status::Ok, Json((request_id, Ok(comparison)).into()))
        },
    }
}
