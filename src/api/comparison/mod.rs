pub(crate) mod handler;

use std::path::Path;

use rand::Rng;
use rocket::{
    http::{
        uri::Origin,
        RawStr,
    },
    serde::uuid::Uuid,
};
use serde::Serialize;

use crate::StaticDir;

#[derive(Serialize)]
pub(crate) struct Comparison<'a> {
    pub(crate) id: Uuid,
    pub(crate) images: Vec<Origin<'a>>,
}

fn get_comparison(
    static_dir: &StaticDir,
) -> Result<Comparison<'_>, sqlx::Error> {
    let mut images = Vec::<Origin<'_>>::new();
    for _ in 0..2 {
        let file_name = get_random_image_file_name(&static_dir.path)?;
        let encoded = RawStr::new(&file_name).percent_encode();
        let origin =
            Origin::parse_owned(format!("{}/{}", static_dir.origin, encoded))
                .map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
            })?;
        images.push(origin);
    }

    let comparison = Comparison {
        id: Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap(),
        images,
    };

    Ok(comparison)
}

fn get_random_image_file_name<P: AsRef<Path>>(
    source: P,
) -> Result<String, std::io::Error> {
    let images: Vec<String> = std::fs::read_dir(source)?
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
        let error = "Not enough files in STATIC_FILES_DIR";
        error!("{}", error);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, error));
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
        let expected_error =
            Err("Not enough files in STATIC_FILES_DIR".to_string());
        assert_eq!(error.map_err(|error| error.to_string()), expected_error);
    }

    #[test]
    fn get_random_image_file_name_when_dir_is_nonexistent() {
        let source = relative!("tests/test_static_dirs/nonexistent");
        let error = super::get_random_image_file_name(&source);
        let expected_error =
            Err("No such file or directory (os error 2)".to_string());
        assert_eq!(error.map_err(|error| error.to_string()), expected_error);
    }
}
