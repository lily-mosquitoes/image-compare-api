mod logger;

use std::{
    path::PathBuf,
    str::FromStr,
};

use image_compare_api;
use log::error;
use sqlx::sqlite::SqliteConnectOptions;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    if let Err(error) = crate::logger::setup() {
        error!("{}", error);
        panic!("failed to setup logger");
    }

    dotenvy::dotenv().expect(".env to be present");

    let allowed_origin = std::env::var("ALLOWED_ORIGIN")
        .expect("`ALLOWED_ORIGIN` to be set in .env");

    let static_dir =
        std::env::var("STATIC_DIR").expect("`STATIC_DIR` to be set in .env");
    let static_dir = PathBuf::from(static_dir);

    let connection_options = std::env::var("DATABASE_URL")
        .expect("`DATABASE_URL` to be set in .env");
    let connection_options =
        SqliteConnectOptions::from_str(&connection_options)
            .expect("Url to be valid");

    let _rocket = image_compare_api::rocket(
        allowed_origin,
        static_dir,
        connection_options,
    )
    .ignite()
    .await?
    .launch()
    .await?;

    Ok(())
}
