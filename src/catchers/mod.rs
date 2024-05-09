use rocket::serde::json::Json;

use crate::response::ResponseBody;

#[catch(404)]
pub(crate) async fn not_found() -> Json<ResponseBody<(), String>> {
    Json(Err("Resource not found".to_string()).into())
}
