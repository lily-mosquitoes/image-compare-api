use rocket::{
    http::Header,
    serde::json::Json,
    Request,
};

use crate::{
    api::RequestId,
    response::ResponseBody,
};

#[catch(404)]
pub(crate) async fn not_found(
    request: &Request<'_>,
) -> Json<ResponseBody<(), String>> {
    let request_id = request
        .guard::<&RequestId>()
        .await
        .expect("BUG: RequestId should return Outcome::Success");

    Json((request_id, Err("Resource not found".to_string())).into())
}

#[catch(401)]
pub(crate) async fn unauthorized(request: &Request<'_>) -> Unauthorized {
    let request_id = request
        .guard::<&RequestId>()
        .await
        .expect("BUG: RequestId should return Outcome::Success");

    Unauthorized {
        body: Json((request_id, Err("Unauthorized".to_string())).into()),
        www_authenticate: Header::new("WWW-Authenticate", "Bearer"),
    }
}

#[derive(rocket::response::Responder)]
pub(crate) struct Unauthorized {
    body: Json<ResponseBody<(), String>>,
    www_authenticate: Header<'static>,
}
