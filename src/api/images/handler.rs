use rocket::{
    http::Status,
    serde::json::Json,
};

use super::{
    get_random_image_file_name,
    ImagesToCompare,
    IoError,
};
use crate::Response;

#[get("/images")]
pub(crate) async fn images_to_compare(
) -> (Status, Json<Response<ImagesToCompare, IoError>>) {
    let (status, data, traceback) = match (
        get_random_image_file_name(),
        get_random_image_file_name(),
    ) {
        (Ok(path_to_image1), Ok(path_to_image2)) => (
            Status::Ok,
            Some(ImagesToCompare {
                path_to_image1: format!("/images/{}", path_to_image1),
                path_to_image2: format!("/images/{}", path_to_image2),
            }),
            None,
        ),
        (Ok(_), Err(error)) => {
            (Status::InternalServerError, None, Some(error))
        },
        (Err(error), Ok(_)) => {
            (Status::InternalServerError, None, Some(error))
        },
        (Err(error), Err(_)) => {
            (Status::InternalServerError, None, Some(error))
        },
    };

    let response =
        Response::build().set_data(data).set_traceback(traceback);

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
            response.into_json::<crate::Response<
                super::ImagesToCompare,
                super::IoError,
            >>();
        assert!(body.is_some());
        let data = body.unwrap().data;
        assert!(data.is_some());
        let images_to_compare = data.unwrap();
        assert!(file_exists(&images_to_compare.path_to_image1));
        assert!(file_exists(&images_to_compare.path_to_image2));
    }

    #[test]
    fn get_image_from_file_server() {
        let image_file_name = super::get_random_image_file_name()
            .expect("Image to be found");
        let image_uri = format!("/images/{}", image_file_name);
        let client = Client::tracked(crate::rocket())
            .expect("valid rocket instance");
        let response = client.get(image_uri).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
