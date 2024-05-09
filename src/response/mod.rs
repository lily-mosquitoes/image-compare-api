use chrono::{
    DateTime,
    Utc,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ResponseBody<T, E> {
    pub(crate) request_id: usize,
    pub(crate) timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<E>,
}

impl<T, E> From<Result<T, E>> for ResponseBody<T, E> {
    fn from(result: Result<T, E>) -> Self {
        let (data, error) = match result {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error)),
        };

        Self {
            request_id: 0,
            timestamp: Utc::now(),
            data,
            error,
        }
    }
}

#[cfg(test)]
mod test {
    use super::ResponseBody;
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
}
