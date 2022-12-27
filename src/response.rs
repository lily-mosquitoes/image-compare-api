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
    use chrono::{
        DateTime,
        Utc,
    };

    macro_rules! assert_struct_has_field {
        ($struct:ty, $field:ident : $type:ty) => {
            const _: () = {
                fn mock(s: $struct) {
                    let _: $type = s.$field;
                }
            };
        };
    }

    #[test]
    fn response_has_timestamp() {
        assert_struct_has_field!(
            super::Response<(), ()>,
            timestamp: DateTime<Utc>
        );
    }

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
