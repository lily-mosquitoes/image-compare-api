use rocket::{
    http::Status,
    serde::json::Json,
    State,
};
use rocket_db_pools::Connection;
use uuid::Uuid;

use super::Comparison;
use crate::{
    response::{
        error::ApiError,
        ResponseBody,
        ToStatus,
    },
    DbPool,
    StaticDir,
};

#[get("/user/<id>/comparison")]
pub(crate) async fn get_comparison_for_user<'a>(
    id: Uuid,
    mut connection: Connection<DbPool>,
    static_dir: &State<StaticDir>,
) -> (Status, Json<ResponseBody<Comparison, ApiError<sqlx::Error>>>) {
    let user = crate::api::user::get_user(id, &mut **connection).await;
    let comparison =
        super::get_comparison_for_user(id, &mut **connection, static_dir).await;

    match (user, comparison) {
        (Err(error), _) => (error.to_status(), Json(Err(error.into()).into())),
        (Ok(_), Err(sqlx::Error::RowNotFound)) => (
            Status::ServiceUnavailable,
            Json(Err(sqlx::Error::RowNotFound.into()).into()),
        ),
        (Ok(_), Err(error)) => {
            (error.to_status(), Json(Err(error.into()).into()))
        },
        (Ok(_), Ok(comparison)) => (Status::Ok, Json(Ok(comparison).into())),
    }
}
