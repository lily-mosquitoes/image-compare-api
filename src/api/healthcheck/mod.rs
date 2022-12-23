pub(crate) mod handler;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Healthcheck {
    message: String,
}
