use rocket::{
    http::Header,
    serde::json::Json,
};

use crate::response::ResponseBody;

#[catch(404)]
pub(crate) async fn not_found() -> Json<ResponseBody<(), String>> {
    Json(Err("Resource not found".to_string()).into())
}

#[catch(401)]
pub(crate) async fn unauthorized() -> Unauthorized {
    Unauthorized {
        body: Json(Err("Unauthorized".to_string()).into()),
        www_authenticate: Header::new("WWW-Authenticate", "Bearer"),
    }
}

#[derive(rocket::response::Responder)]
struct Unauthorized {
    body: Json<ResponseBody<(), String>>,
    www_authenticate: Header<'static>,
}
