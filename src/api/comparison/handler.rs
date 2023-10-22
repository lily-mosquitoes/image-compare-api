use rocket::{
    http::{
        uri::Origin,
        RawStr,
        Status,
    },
    serde::{
        json::Json,
        uuid::Uuid,
    },
    State,
};
use serde::Serialize;

use super::{
    get_random_image_file_name,
    IoError,
};
use crate::{
    response::Response,
    StaticDir,
};

#[derive(Serialize)]
pub(crate) struct Comparison<'a> {
    pub(crate) id: Uuid,
    pub(crate) images: Vec<Origin<'a>>,
}

impl<'a> From<Vec<Origin<'a>>> for Comparison<'a> {
    fn from(value: Vec<Origin<'a>>) -> Self {
        Comparison {
            // TODO: implement database
            id: Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6")
                .unwrap(),
            images: value,
        }
    }
}

impl Into<IoError> for rocket::http::uri::Error<'_> {
    fn into(self) -> IoError {
        IoError::FileServerError(format!("{}", self))
    }
}

#[get("/comparison")]
pub(crate) async fn comparison<'a>(
    static_dir: &State<StaticDir>,
) -> (Status, Json<Response<Comparison<'a>, IoError>>) {
    let images: Result<Vec<Origin<'a>>, IoError> = (0..2)
        .map(|_| get_random_image_file_name(&static_dir.path))
        .map(|r| {
            r.and_then(|f| {
                let encoded = RawStr::new(&f).percent_encode();
                Origin::parse_owned(format!(
                    "{}/{}",
                    &static_dir.origin, encoded
                ))
                .map_err(|e| e.into())
            })
        })
        .collect();

    let (status, data) = match images {
        Ok(images) => (Status::Ok, Ok(Comparison::from(images))),
        Err(error) => (Status::ServiceUnavailable, Err(error)),
    };

    let response = Response::from_result(data);

    (status, Json(response))
}
