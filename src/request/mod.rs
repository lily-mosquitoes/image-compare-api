use std::fmt;

use rocket::http::Status;
use serde::{
    Serialize,
    Serializer,
};

#[derive(Debug)]
pub(crate) enum RequestError {
    ResourceNotFound,
    InvalidRequestParameter(String),
    InvalidRequestBody(Vec<String>),
    DatabaseError(String),
    InternalServerError,
    ServiceUnavailable,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResourceNotFound => write!(f, "Resource not found."),
            _ => write!(f, ""),
        }
    }
}

impl std::error::Error for RequestError {}

impl Serialize for RequestError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl RequestError {
    /// Returns the http error status for this request error
    fn status(&self) -> Status {
        match self {
            Self::ResourceNotFound => Status::NotFound,
            Self::DatabaseError(error) => Status::UnprocessableEntity,
            _ => Status::InternalServerError,
        }
    }
}

#[cfg(test)]
mod test {
    use rocket::{
        http::Status,
        serde::json,
    };

    #[test]
    fn serialize_resource_not_found() {
        let error = super::RequestError::ResourceNotFound;
        let json_error = json::to_string(&error).unwrap();
        let result: String = json::from_str(&json_error).unwrap();
        assert_eq!(result, "Resource not found");
    }

    #[test]
    fn resource_not_found_is_status_404() {
        let error = super::RequestError::ResourceNotFound;
        let status = error.status();
        assert_eq!(status, Status::NotFound);
    }

    #[test]
    fn invalid_request_parameter_is_status_422() {
        let error = super::RequestError::InvalidRequestParameter(
            "id must be uuid".to_string(),
        );
        let status = error.status();
        assert_eq!(status, Status::UnprocessableEntity);
    }

    #[test]
    fn invalid_request_body_is_status_400() {
        let error = super::RequestError::InvalidRequestBody(vec![
            "test_a".to_string(),
            "test_b".to_string(),
        ]);
        let status = error.status();
        assert_eq!(status, Status::BadRequest);
    }

    #[test]
    fn database_error_is_status_422() {
        let error = super::RequestError::DatabaseError(
            "IntegrityError: id must be unique".to_string(),
        );
        let status = error.status();
        assert_eq!(status, Status::UnprocessableEntity);
    }

    #[test]
    fn internal_server_error_is_status_500() {
        let error = super::RequestError::InternalServerError;
        let status = error.status();
        assert_eq!(status, Status::InternalServerError);
    }
}
