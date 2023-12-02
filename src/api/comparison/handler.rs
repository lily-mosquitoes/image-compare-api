use rocket::{
    http::Status,
    serde::json::Json,
    State,
};

use super::Comparison;
use crate::{
    request::RequestError,
    response::Response,
    StaticDir,
};

#[get("/comparison")]
pub(crate) async fn get_comparison<'a>(
    static_dir: &'a State<StaticDir>,
) -> (Status, Json<Response<Comparison<'a>, RequestError<sqlx::Error>>>) {
    let result =
        super::get_comparison(&static_dir).map_err(|error| error.into());
    let response = Response::from_result(result);

    (response.status(), Json(response))
}
