pub(crate) mod handler;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ImagesToCompare {
    path_to_image1: String,
    path_to_image2: String,
}
