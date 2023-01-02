mod logger;

use image_compare_api;
use log::error;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    if let Err(error) = crate::logger::setup() {
        error!("{}", error);
        panic!("failed to setup logger");
    }

    let _rocket =
        image_compare_api::rocket().ignite().await?.launch().await?;

    Ok(())
}
