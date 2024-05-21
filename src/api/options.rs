use rocket::response::status::NoContent;

#[options("/options")]
pub(crate) async fn options() -> NoContent {
    NoContent
}
