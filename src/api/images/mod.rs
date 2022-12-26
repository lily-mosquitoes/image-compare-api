pub(crate) mod handler;

use rand::Rng;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ImagesToCompare {
    path_to_image1: String,
    path_to_image2: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum IoError {
    OsError(String),
    FileServerError(String),
}

pub(crate) fn get_random_image_file_name() -> Result<String, IoError>
{
    let images: Vec<String> =
        std::fs::read_dir(crate::STATIC_FILES_DIR)
            .map_err(|error| {
                IoError::OsError(error.kind().to_string())
            })?
            .filter_map(|x| x.ok())
            .map(|x| x.file_name().into_string())
            .filter_map(|x| match x {
                Ok(value) => Some(value),
                Err(error) => {
                    error!("Invalid UTF Character in: {:?}", error);
                    None
                },
            })
            .collect();

    if images.len() <= 0 {
        let error = "Empty STATIC_FILES_DIR".to_string();
        error!("{}", error);
        return Err(IoError::FileServerError(error));
    }

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..images.len());

    Ok(images[index].to_owned())
}

#[cfg(test)]
mod test {
    use std::{
        ffi::OsString,
        fs,
    };

    fn file_exists(file_name: &str) -> bool {
        let entries: Vec<OsString> =
            fs::read_dir(crate::STATIC_FILES_DIR)
                .expect(
                    "`STATIC_FILES_DIR` to exist and be accessible",
                )
                .filter_map(|x| x.ok())
                .map(|x| x.file_name())
                .collect();

        entries.contains(&OsString::from(file_name))
    }

    #[test]
    fn get_random_image_file_name() {
        let file_name = super::get_random_image_file_name()
            .expect("random image to be found");
        assert!(file_exists(&file_name));
    }
}
