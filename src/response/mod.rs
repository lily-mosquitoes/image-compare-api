use chrono::{
    DateTime,
    Utc,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Response<T, E> {
    pub(crate) request_id: usize,
    pub(crate) timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<E>,
}

impl<T, E> Response<T, E> {
    pub(crate) fn from_result(result: Result<T, E>) -> Self {
        let (data, error) = match result {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error)),
        };

        Response::<T, E> {
            request_id: 0,
            timestamp: Utc::now(),
            data,
            error,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn make_response_from_ok_result() {
        let result: Result<i8, u8> = Ok(-2);
        let response = super::Response::from_result(result);
        assert!(response.data.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn make_response_from_err_result() {
        let result: Result<i8, u8> = Err(2);
        let response = super::Response::from_result(result);
        assert!(response.data.is_none());
        assert!(response.error.is_some());
    }
}
