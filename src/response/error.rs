use std::{
    error::Error,
    fmt::Display,
};

use rocket::http::Status;
use serde::{
    Serialize,
    Serializer,
};

use super::ToStatus;

#[derive(Debug)]
pub(crate) struct ApiError<T: Display + Error + ToStatus> {
    pub(crate) inner: T,
}

impl<T: Display + Error + ToStatus> Display for ApiError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T: Display + Error + ToStatus> Error for ApiError<T> {}

impl<T: Display + Error + ToStatus> ToStatus for ApiError<T> {
    fn to_status(&self) -> Status {
        self.inner.to_status()
    }
}

impl<T: Display + Error + ToStatus> From<T> for ApiError<T> {
    fn from(value: T) -> Self {
        Self { inner: value }
    }
}

impl<T: Display + Error + ToStatus> Serialize for ApiError<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
