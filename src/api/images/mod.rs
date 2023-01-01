pub(crate) mod handler;

use std::fmt;

use rand::Rng;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Image {
    id: i64,
    src: String,
}

impl Image {
    pub(crate) fn new(id: i64, src: String) -> Self {
        Image { id, src }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ImagesToCompare {
    image1: Image,
    image2: Image,
}

impl ImagesToCompare {
    pub(crate) fn new(image1: Image, image2: Image) -> Self {
        ImagesToCompare { image1, image2 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum IoError {
    OsError(String),
    FileServerError(String),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::OsError(e) => write!(f, "OsError: {}", e),
            IoError::FileServerError(e) => {
                write!(f, "FileServerError: {}", e)
            },
        }
    }
}

fn get_random_image_file_name() -> Result<String, IoError> {
    let static_files_dir = &*crate::statics::STATIC_FILES_DIR;

    let images: Vec<String> = std::fs::read_dir(static_files_dir)
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

    if images.len() <= 0 {
        let error = "Empty STATIC_FILES_DIR".to_string();
        error!("{}", error);
        return Err(IoError::FileServerError(error));
    }

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..images.len());

    Ok(images[index].to_owned())
}

fn get_random_image() -> Result<Image, IoError> {
    let src = get_random_image_file_name()?;

    Ok(Image::new(1, src))
}

pub(crate) fn get_random_images_to_compare(
) -> Result<ImagesToCompare, IoError> {
    let image1 = get_random_image()?;
    let image2 = get_random_image()?;

    Ok(ImagesToCompare::new(image1, image2))
}

#[cfg(test)]
mod test {
    use crate::test_helpers::file_exists;

    #[test]
    fn get_random_image_file_name() {
        let file_name = super::get_random_image_file_name()
            .expect("random image file name to be found");
        assert!(file_exists(&file_name));
    }

    #[test]
    fn make_image() {
        let id = 1;
        let src = "/images/example.png".to_string();
        let image = super::Image::new(id, src.clone());
        assert_eq!(image.id, id);
        assert_eq!(image.src, src);
    }

    #[test]
    fn get_random_image() {
        let image = super::get_random_image()
            .expect("random image to be found");
        assert!(file_exists(&image.src));
    }

    #[test]
    fn make_images_to_compare() {
        let image1_id = 1;
        let image1_src = "/images/example1.png".to_string();
        let image2_id = 2;
        let image2_src = "/images/example2.png".to_string();
        let image1 = super::Image::new(image1_id, image1_src);
        let image2 = super::Image::new(image2_id, image2_src);
        let images_to_compare = super::ImagesToCompare::new(
            image1.clone(),
            image2.clone(),
        );
        assert_eq!(images_to_compare.image1, image1);
        assert_eq!(images_to_compare.image2, image2);
    }

    #[test]
    fn get_random_images_to_compare() {
        let images_to_compare = super::get_random_images_to_compare()
            .expect("random images to be found");
        assert!(file_exists(&images_to_compare.image1.src));
        assert!(file_exists(&images_to_compare.image2.src));
    }
}
