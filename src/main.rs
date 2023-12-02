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
    let static_dir =
        std::env::var("STATIC_DIR").expect("`STATIC_DIR to be set in .env`");
    let static_dir = PathBuf::from(static_dir);

    let connection_options =
        SqliteConnectOptions::from_str("sqlite://sqlite.db")
            .expect("Url to be valid");

    let _rocket = image_compare_api::rocket(static_dir, connection_options)
        .ignite()
        .await?
        .launch()
        .await?;

    Ok(())
}
