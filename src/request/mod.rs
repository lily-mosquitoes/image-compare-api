use std::{
    error::Error,
    fmt::Display,
};

use rocket::http::Status;
use serde::{
    Serialize,
    Serializer,
};

use crate::response::ToStatus;

#[derive(Debug)]
pub(crate) struct RequestError<T: Display + Error + ToStatus> {
    pub(crate) inner: T,
}

impl<T: Display + Error + ToStatus> Display for RequestError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T: Display + Error + ToStatus> Error for RequestError<T> {}

impl<T: Display + Error + ToStatus> Serialize for RequestError<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<T: Display + Error + ToStatus> ToStatus for RequestError<T> {
    fn to_status(&self) -> Status {
        self.inner.to_status()
    }
}

impl<T: Display + Error + ToStatus> From<T> for RequestError<T> {
    fn from(value: T) -> Self {
        Self { inner: value }
    }
}

// #[cfg(test)]
// mod test {
//     use rocket::{http::Status, serde::json};
//
//     #[test]
//     fn serialize_resource_not_found() {
//         let error = super::RequestError::ResourceNotFound;
//         let json_error = json::to_string(&error).unwrap();
//         let result: String = json::from_str(&json_error).unwrap();
//         assert_eq!(result, "Resource not found");
//     }
//
//     #[test]
//     fn resource_not_found_is_status_404() {
//         let error = super::RequestError::ResourceNotFound;
//         let status = error.status();
//         assert_eq!(status, Status::NotFound);
//     }
//
//     #[test]
//     fn invalid_request_parameter_is_status_422() {
//         let error = super::RequestError::InvalidRequestParameter(
//             "id must be uuid".to_string(),
//         );
//         let status = error.status();
//         assert_eq!(status, Status::UnprocessableEntity);
//     }
//
//     #[test]
//     fn invalid_request_body_is_status_400() {
//         let error = super::RequestError::InvalidRequestBody(vec![
//             "test_a".to_string(),
//             "test_b".to_string(),
//         ]);
//         let status = error.status();
//         assert_eq!(status, Status::BadRequest);
//     }
//
//     #[test]
//     fn database_error_is_status_422() {
//         let error = super::RequestError::DatabaseError(
//             "IntegrityError: id must be unique".to_string(),
//         );
//         let status = error.status();
//         assert_eq!(status, Status::UnprocessableEntity);
//     }
//
//     #[test]
//     fn internal_server_error_is_status_500() {
//         let error = super::RequestError::InternalServerError;
//         let status = error.status();
//         assert_eq!(status, Status::InternalServerError);
//     }
// }
