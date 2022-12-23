use chrono::{
    DateTime,
    Utc,
};
use rocket::{
    http::Status,
    serde::json::Json,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Response<T> {
    timestamp: DateTime<Utc>,
    // request_id: RequestId,
    // traceback: Option<Error>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Healthcheck {
    message: String,
}

#[get("/healthcheck")]
pub(crate) async fn healthcheck(
) -> (Status, Json<Response<Healthcheck>>) {
    let response = Response {
        timestamp: Utc::now(),
        data: None,
    };

    (Status::Ok, Json(response))
}

#[cfg(test)]
mod test {
    use rocket::{
        http::Status,
        local::blocking::Client,
    };

    #[test]
    fn healthcheck() {
        let client = Client::tracked(crate::rocket())
            .expect("valid rocket instance");
        let response =
            client.get(uri!("/api", super::healthcheck)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response
            .into_json::<super::Response<super::Healthcheck>>();
        assert!(body.is_some());
    }
}
