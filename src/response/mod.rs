use chrono::{
    DateTime,
    Utc,
};
use rocket::http::Status;
use serde::Serialize;

pub(crate) mod error;

#[derive(Debug, Serialize)]
pub(crate) struct ResponseBody<T, E: ToStatus> {
    pub(crate) request_id: usize,
    pub(crate) timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<E>,
    #[serde(skip_serializing)]
    success_status: Status,
}

impl<T, E: ToStatus> From<Result<T, E>> for ResponseBody<T, E> {
    fn from(result: Result<T, E>) -> Self
    where
        E: ToStatus,
    {
        let (data, error) = match result {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error)),
        };

        Self {
            request_id: 0,
            timestamp: Utc::now(),
            data,
            error,
            success_status: Status::Ok,
        }
    }
}

impl<T, E: ToStatus> ResponseBody<T, E> {
    pub(crate) fn with_success_status(mut self, status: Status) -> Self {
        self.success_status = status;
        self
    }

    pub(crate) fn status(&self) -> Status {
        self.error
            .as_ref()
            .and_then(|error| Some(error.to_status()))
            .unwrap_or(self.success_status)
    }
}

pub(crate) trait ToStatus {
    fn to_status(&self) -> Status;
}

impl ToStatus for () {
    fn to_status(&self) -> Status {
        Status::InternalServerError
    }
}

impl ToStatus for sqlx::Error {
    fn to_status(&self) -> Status {
        match self {
            Self::RowNotFound => Status::NotFound,
            Self::Io(_) => Status::ServiceUnavailable,
            _ => Status::InternalServerError,
        }
    }
}

#[cfg(test)]
mod test {
    use rocket::http::Status;

    use super::{
        ResponseBody,
        ToStatus,
    };

    impl ToStatus for u8 {
        fn to_status(&self) -> Status {
            Status::InternalServerError
        }
    }

    #[test]
    fn make_response_body_from_ok_result() {
        let result: Result<i8, u8> = Ok(-2);
        let response_body = ResponseBody::from(result);
        assert!(response_body.data.is_some());
        assert!(response_body.error.is_none());
    }

    #[test]
    fn make_response_body_from_err_result() {
        let result: Result<i8, u8> = Err(2);
        let response_body = ResponseBody::from(result);
        assert!(response_body.data.is_none());
        assert!(response_body.error.is_some());
    }

    #[test]
    fn make_response_body_with_200_ok_success_status() {
        let result: Result<i8, u8> = Err(2);
        let response_body = ResponseBody::from(result);
        assert_eq!(response_body.success_status, Status::Ok);
    }

    #[test]
    fn make_response_body_with_different_success_status() {
        let result: Result<i8, u8> = Err(2);
        let response_body =
            ResponseBody::from(result).with_success_status(Status::Created);
        assert_eq!(response_body.success_status, Status::Created);
    }

    #[test]
    fn get_status_from_ok_response_body() {
        let result: Result<i8, u8> = Ok(-2);
        let response_body =
            ResponseBody::from(result).with_success_status(Status::Created);
        assert_eq!(response_body.status(), Status::Created);
    }

    #[test]
    fn get_status_from_err_response_body() {
        let result: Result<i8, ()> = Err(());
        let response_body =
            ResponseBody::from(result).with_success_status(Status::Created);
        assert_eq!(response_body.status(), Status::InternalServerError);
    }
}
