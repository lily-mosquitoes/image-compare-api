mod logger;

use std::path::PathBuf;

use image_compare_api;
use log::error;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    if let Err(error) = crate::logger::setup() {
        error!("{}", error);
        panic!("failed to setup logger");
    }

    dotenvy::dotenv().expect(".env to be present");
    let static_dir = std::env::var("STATIC_DIR")
        .expect("`STATIC_FILES_DIR to be set in .env`");
    let static_dir = PathBuf::from(static_dir);

    let _rocket = image_compare_api::rocket(static_dir)
        .ignite()
        .await?
        .launch()
        .await?;

    Ok(())
}
