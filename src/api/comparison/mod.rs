pub(crate) mod handler;

use std::{
    error::Error,
    fmt,
    path::Path,
};

use rand::Rng;
use serde::{
    Serialize,
    Serializer,
};

#[derive(Debug, PartialEq)]
pub(crate) enum IoError {
    OsError(String),
    FileServerError(String),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OsError(e) => write!(f, "OsError: {}", e),
            Self::FileServerError(e) => {
                write!(f, "FileServerError: {}", e)
            },
        }
    }
}

impl Error for IoError {}

impl Serialize for IoError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

fn get_random_image_file_name<P: AsRef<Path>>(
    source: P,
) -> Result<String, IoError> {
    let images: Vec<String> = std::fs::read_dir(source)
        .map_err(|error| IoError::OsError(error.kind().to_string()))?
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

    if images.len() < 2 {
        let error = "Not enough files in STATIC_FILES_DIR".to_string();
        error!("{}", error);
        return Err(IoError::FileServerError(error));
    }

    let index = rand::thread_rng().gen_range(0..images.len());

    Ok(images[index].to_owned())
}

#[cfg(test)]
mod test {
    use std::{
        ffi::OsString,
        path::Path,
    };

    use rocket::fs::relative;

    fn file_exists<P: AsRef<Path>>(source: P, file_name: &str) -> bool {
        let entries: Vec<OsString> = std::fs::read_dir(source)
            .expect("`STATIC_FILES_DIR` to exist and be accessible")
            .filter_map(|x| x.ok())
            .map(|x| x.file_name())
            .collect();

        entries.contains(&OsString::from(file_name))
    }

    #[test]
    fn get_random_image_file_name_when_dir_has_2_files() {
        let source = relative!("tests/test_static_dirs/with_2_files");
        let file_name = super::get_random_image_file_name(&source)
            .expect("random image file name to be found");
        assert!(file_exists(&source, &file_name));
    }

    #[test]
    fn get_random_image_file_name_when_dir_has_1_file() {
        let source = relative!("tests/test_static_dirs/with_1_file");
        let error = super::get_random_image_file_name(&source);
        let expected_error = Err(super::IoError::FileServerError(
            "Not enough files in STATIC_FILES_DIR".to_string(),
        ));
        assert_eq!(error, expected_error);
    }

    #[test]
    fn get_random_image_file_name_when_dir_is_empty() {
        let source = relative!("tests/test_static_dirs/empty");
        let error = super::get_random_image_file_name(&source);
        let expected_error = Err(super::IoError::FileServerError(
            "Not enough files in STATIC_FILES_DIR".to_string(),
        ));
        assert_eq!(error, expected_error);
    }

    #[test]
    fn get_random_image_file_name_when_dir_is_nonexistent() {
        let source = relative!("tests/test_static_dirs/nonexistent");
        let error = super::get_random_image_file_name(&source);
        let expected_error =
            Err(super::IoError::OsError("entity not found".to_string()));
        assert_eq!(error, expected_error);
    }
}
