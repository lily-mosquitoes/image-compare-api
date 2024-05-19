use chrono::{
    DateTime,
    Utc,
};
use serde::Serialize;

use crate::api::RequestId;

#[derive(Debug, Serialize)]
pub(crate) struct ResponseBody<T, E> {
    pub(crate) request_id: RequestId,
    pub(crate) timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<E>,
}

impl<T, E> From<(&RequestId, Result<T, E>)> for ResponseBody<T, E> {
    fn from(value: (&RequestId, Result<T, E>)) -> Self {
        let (request_id, data, error) = match value {
            (request_id, Ok(data)) => (*request_id, Some(data), None),
            (request_id, Err(error)) => (*request_id, None, Some(error)),
        };

        Self {
            request_id,
            timestamp: Utc::now(),
            data,
            error,
        }
    }
}

#[cfg(test)]
mod test {
    use super::ResponseBody;
    use crate::api::RequestId;

    #[test]
    fn make_response_body_from_ok_result() {
        let result: Result<i8, u8> = Ok(-2);
        let response_body = ResponseBody::from((&RequestId(0), result));
        assert!(response_body.data.is_some());
        assert!(response_body.error.is_none());
    }

    #[test]
    fn make_response_body_from_err_result() {
        let result: Result<i8, u8> = Err(2);
        let response_body = ResponseBody::from((&RequestId(0), result));
        assert!(response_body.data.is_none());
        assert!(response_body.error.is_some());
    }
}
