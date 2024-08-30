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

#[get("/user/<id>/comparison?<dirname>")]
pub(crate) async fn get_comparison_for_user<'r>(
    id: Uuid,
    request_id: &RequestId,
    dirname: Option<String>,
    mut connection: Connection<DbPool>,
) -> (Status, Json<ResponseBody<Comparison<'r>, QueryError>>) {
    let user = crate::api::user::get_user(id, &mut **connection).await;
    let dirname = dirname.unwrap_or("".to_string());
    let comparison =
        super::get_comparison_for_user(id, dirname, &mut **connection).await;

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

#[get("/comparison/dirnames")]
pub(crate) async fn get_comparison_dirnames<'r>(
    request_id: &RequestId,
    mut connection: Connection<DbPool>,
) -> (Status, Json<ResponseBody<Vec<String>, QueryError>>) {
    let dirnames = super::get_comparison_dirnames(&mut **connection).await;

    match dirnames {
        Err(error) => {
            (error.default_status(), Json((request_id, Err(error)).into()))
        },
        Ok(dirnames) if dirnames.len() == 0 => {
            let error = QueryError::RowNotFound(
                "No `comparison`s available".to_string(),
            );
            (Status::ServiceUnavailable, Json((request_id, Err(error)).into()))
        },
        Ok(dirnames) => (Status::Ok, Json((request_id, Ok(dirnames)).into())),
    }
}
