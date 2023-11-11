pub(crate) mod handler;

use std::fmt;

use rocket::{
    http::Status,
    serde::uuid::Uuid,
};
use serde::Serialize;

use crate::request::RequestError;

#[derive(Serialize)]
pub(crate) struct User {
    pub(crate) id: Uuid,
    pub(crate) comparisons: u64,
    pub(crate) average_lambda: f64,
}

#[derive(Debug, PartialEq)]
pub(crate) enum TransactionError {
    RowNotFound,
    DatabaseError(String),
    Io(String),
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RowNotFound => write!(f, "Row not found"),
            Self::DatabaseError(error) => {
                write!(f, "Database error: {}", error)
            },
            Self::Io(error) => write!(f, "I/O error: {}", error),
        }
    }
}

impl std::error::Error for TransactionError {}

pub(crate) trait ToStatus {
    fn to_status(&self) -> (Status, RequestError);
}

impl ToStatus for TransactionError {
    fn to_status(&self) -> (Status, RequestError) {
        match self {
            Self::RowNotFound => {
                (Status::NotFound, RequestError::ResourceNotFound)
            },
            Self::DatabaseError(error) => (
                Status::UnprocessableEntity,
                RequestError::DatabaseError(*error),
            ),
            Self::Io(error) => {
                error!("TransactionError: {}", error);
                (Status::InternalServerError, RequestError::InternalServerError)
            },
        }
    }
}

pub(crate) fn get_user(id: Uuid) -> Result<User, TransactionError> {
    let user = User {
        id: Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap(),
        comparisons: 0,
        average_lambda: 0.0,
    };
    Ok(user)
}
