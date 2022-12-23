use chrono::Utc;
use rocket::{
    http::Status,
    serde::json::Json,
};

use super::ImagesToCompare;
use crate::Response;

#[get("/images")]
pub(crate) async fn images_to_compare(
) -> (Status, Json<Response<ImagesToCompare>>) {
    let images = ImagesToCompare {
        path_to_image1: "".to_string(),
        path_to_image2: "".to_string(),
    };

    let response = Response {
        timestamp: Utc::now(),
        data: Some(images),
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
        let response = client
            .get(uri!("/api", super::images_to_compare))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response
            .into_json::<crate::Response<super::ImagesToCompare>>();
        assert!(body.is_some());
        let data = body.unwrap().data;
        assert!(data.is_some());
    }
}
