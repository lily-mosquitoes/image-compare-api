use rocket::{
    http::Status,
    serde::json::Json,
};

use super::{
    get_random_images_to_compare,
    ImagesToCompare,
    IoError,
};
use crate::response::Response;

#[get("/images")]
pub(crate) async fn images_to_compare(
) -> (Status, Json<Response<ImagesToCompare, IoError>>) {
    let (status, data) = match get_random_images_to_compare() {
        Ok(images) => (Status::Ok, Ok(images)),
        Err(error) => {
            error!("{}", error);
            (Status::InternalServerError, Err(error))
        },
    };

    let response = Response::from_result(data);

    (status, Json(response))
}
