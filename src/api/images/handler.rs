use rocket::{
    http::Status,
    serde::json::Json,
};

use super::{
    get_random_images_to_compare,
    ImagesToCompare,
    IoError,
};
use crate::Response;

#[get("/images")]
pub(crate) async fn images_to_compare(
) -> (Status, Json<Response<ImagesToCompare, IoError>>) {
    let (status, data) = match get_random_images_to_compare() {
        Ok(images) => (Status::Ok, Ok(images)),
        Err(error) => {
            error!("{}", error);
            (Status::InternalServerError, Err(error))
        },
    };

    let response = Response::new_with_data(data);

    (status, Json(response))
}

#[cfg(test)]
mod test {
    use std::{
        ffi::OsString,
        fs,
    };

    use rocket::{
        http::Status,
        local::blocking::Client,
    };

    fn file_exists(file_name: &str) -> bool {
        let static_files_dir = crate::STATIC_FILES_DIR;

        let entries: Vec<OsString> = fs::read_dir(static_files_dir)
            .expect("`STATIC_FILES_DIR` to exist and be accessible")
            .filter_map(|x| x.ok())
            .map(|x| x.file_name())
            .collect();

        entries.contains(&OsString::from(
            file_name.replace("/images/", ""),
        ))
    }

    #[test]
    fn get_images_to_compare() {
        let client = Client::tracked(crate::rocket())
            .expect("valid rocket instance");
        let response = client
            .get(uri!("/api", super::images_to_compare))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body =
            response
                .into_json::<crate::Response<
                    super::ImagesToCompare,
                    super::IoError,
                >>()
                .expect("body to be present");
        assert!(body.data.is_some());
        assert!(body.error.is_none());
        let images_to_compare = body.data.unwrap();
        assert!(file_exists(&images_to_compare.image1.src));
        assert!(file_exists(&images_to_compare.image2.src));
    }

    #[test]
    fn get_image_from_file_server() {
        let images = super::get_random_images_to_compare()
            .expect("Images to be found");
        let image_uri = format!("/images/{}", images.image1.src);
        let client = Client::tracked(crate::rocket())
            .expect("valid rocket instance");
        let response = client.get(image_uri).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
