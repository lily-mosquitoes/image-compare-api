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
    // request_id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) traceback: Option<E>,
}

impl<T, E> Response<T, E> {
    pub(crate) fn build() -> Self {
        Response::<T, E> {
            timestamp: Utc::now(),
            data: None,
            traceback: None,
        }
    }

    pub(crate) fn set_data(mut self, data: Option<T>) -> Self {
        self.data = data;
        self
    }

    pub(crate) fn set_traceback(
        mut self,
        traceback: Option<E>,
    ) -> Self {
        self.traceback = traceback;
        self
    }
}
