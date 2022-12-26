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
    pub(crate) data: Result<T, E>,
}

impl<T, E> Response<T, E> {
    pub(crate) fn new_with_data(data: Result<T, E>) -> Self {
        Response::<T, E> {
            timestamp: Utc::now(),
            data,
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
        assert!(ok.data.is_ok());
        assert!(err.data.is_err());
    }
}
