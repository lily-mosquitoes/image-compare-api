pub(crate) mod handler;

use rocket::{
    http::uri::Origin,
    State,
};
use serde::Serialize;
use sqlx::SqliteConnection;
use uuid::Uuid;

use super::{
    SqliteArray,
    SqliteUuid,
};
use crate::StaticDir;

#[derive(Serialize)]
pub(crate) struct Comparison<'a> {
    pub(crate) id: Uuid,
    pub(crate) images: Vec<Origin<'a>>,
}

struct ComparisonRaw {
    id: SqliteUuid,
    images: SqliteArray,
}

async fn get_comparison_for_user(
    user_id: Uuid,
    connection: &mut SqliteConnection,
    static_dir: &State<StaticDir>,
) -> Result<Comparison<'static>, sqlx::Error> {
    let comparison = sqlx::query_as!(
        ComparisonRaw,
        "SELECT * FROM comparison WHERE id NOT IN (SELECT comparison_id FROM \
         vote WHERE user_id = ?) LIMIT 1",
        user_id
    )
    .fetch_one(connection)
    .await?;

    let images = comparison
        .images
        .iter()
        .map(|image_filename| {
            Origin::parse_owned(format!(
                "{}/{}",
                static_dir.origin, image_filename
            ))
            .expect("BUG: image path should be parseable to origin.")
        })
        .collect();

    Ok(Comparison {
        id: *comparison.id,
        images,
    })
}
