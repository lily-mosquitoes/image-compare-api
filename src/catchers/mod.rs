use std::{
    error::Error,
    fmt,
};

use rocket::serde::json::Json;
use serde::{
    Serialize,
    Serializer,
};

use crate::response::Response;

#[derive(Debug)]
pub(crate) enum RequestError {
    ResourceNotFound(String),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResourceNotFound(e) => write!(f, "ResourceNotFound: {}", e),
        }
    }
}

impl Error for RequestError {}

impl Serialize for RequestError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[catch(404)]
pub(crate) async fn not_found(
    request: &rocket::Request<'_>,
) -> Json<Response<(), RequestError>> {
    let result = Err(RequestError::ResourceNotFound(request.uri().to_string()));

    Json(Response::from_result(result))
}
