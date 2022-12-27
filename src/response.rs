use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Response<T, E> {
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
            timestamp: Utc::now(),
            data,
            error,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn make_response_from_result() {
        let ok_result: Result<i8, u8> = Ok(-2);
        let err_result: Result<i8, u8> = Err(2);
        let ok_response = super::Response::from_result(ok_result);
        let err_response = super::Response::from_result(err_result);
        assert!(ok_response.data.is_some());
        assert!(ok_response.error.is_none());
        assert!(err_response.data.is_none());
        assert!(err_response.error.is_some());
    }
}
