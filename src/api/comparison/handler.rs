use rocket::{
    http::Status,
    serde::json::Json,
    State,
};
use rocket_db_pools::Connection;
use uuid::Uuid;

use super::Comparison;
use crate::{
    request::RequestError,
    response::Response,
    DbPool,
    StaticDir,
};

#[get("/user/<id>/comparison")]
pub(crate) async fn get_comparison_for_user<'a>(
    id: Uuid,
    mut connection: Connection<DbPool>,
    static_dir: &State<StaticDir>,
) -> (Status, Json<Response<Comparison, RequestError<sqlx::Error>>>) {
    let result =
        super::get_comparison_for_user(id, &mut **connection, static_dir)
            .await
            .map_err(|error| error.into());
    let response = Response::from_result(result);

    (response.status(), Json(response))
}
