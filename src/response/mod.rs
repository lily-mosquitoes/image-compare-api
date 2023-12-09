use chrono::{
    DateTime,
    Utc,
};
use rocket::http::Status;
use serde::Serialize;

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

#[derive(Debug, Serialize)]
pub(crate) struct Response<T, E: ToStatus> {
    pub(crate) request_id: usize,
    pub(crate) timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<E>,
    #[serde(skip_serializing)]
    success_status: Status,
}

impl<T, E: ToStatus> Response<T, E> {
    pub(crate) fn from_result(result: Result<T, E>) -> Self
    where
        E: ToStatus,
    {
        let (data, error) = match result {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error)),
        };

        Response::<T, E> {
            request_id: 0,
            timestamp: Utc::now(),
            data,
            error,
            success_status: Status::Ok,
        }
    }

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

#[cfg(test)]
mod test {
    use rocket::http::Status;

    use super::{
        Response,
        ToStatus,
    };

    impl ToStatus for u8 {
        fn to_status(&self) -> Status {
            Status::InternalServerError
        }
    }

    #[test]
    fn make_response_from_ok_result() {
        let result: Result<i8, u8> = Ok(-2);
        let response = Response::from_result(result);
        assert!(response.data.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn make_response_from_err_result() {
        let result: Result<i8, u8> = Err(2);
        let response = Response::from_result(result);
        assert!(response.data.is_none());
        assert!(response.error.is_some());
    }

    #[test]
    fn make_response_with_200_ok_success_status() {
        let result: Result<i8, u8> = Err(2);
        let response = Response::from_result(result);
        assert_eq!(response.success_status, Status::Ok);
    }

    #[test]
    fn make_response_with_different_success_status() {
        let result: Result<i8, u8> = Err(2);
        let response =
            Response::from_result(result).with_success_status(Status::Created);
        assert_eq!(response.success_status, Status::Created);
    }

    #[test]
    fn get_status_from_ok_response() {
        let result: Result<i8, u8> = Ok(-2);
        let response =
            Response::from_result(result).with_success_status(Status::Created);
        assert_eq!(response.status(), Status::Created);
    }

    #[test]
    fn get_status_from_err_response() {
        let result: Result<i8, ()> = Err(());
        let response =
            Response::from_result(result).with_success_status(Status::Created);
        assert_eq!(response.status(), Status::InternalServerError);
    }
}
