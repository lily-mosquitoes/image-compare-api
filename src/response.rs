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
    pub(crate) fn new_with_data(result: Result<T, E>) -> Self {
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
    fn make_response() {
        let ok_result: Result<i8, u8> = Ok(-2);
        let err_result: Result<i8, u8> = Err(2);
        let ok = super::Response::new_with_data(ok_result);
        let err = super::Response::new_with_data(err_result);
        assert!(ok.data.is_some());
        assert!(ok.error.is_none());
        assert!(err.data.is_none());
        assert!(err.error.is_some());
    }
}
