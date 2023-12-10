pub(crate) mod handler;

use std::ops::Deref;

use rocket::{
    http::uri::Origin,
    State,
};
use serde::Serialize;
use sqlx::SqliteConnection;
use uuid::Uuid;

use super::SqliteUuid;
use crate::StaticDir;

struct SqliteArray(Vec<String>);

impl From<String> for SqliteArray {
    fn from(value: String) -> Self {
        Self(value.split("/").map(str::to_string).collect())
    }
}

impl Deref for SqliteArray {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct ComparisonRaw {
    id: SqliteUuid,
    images: SqliteArray,
}

#[derive(Serialize)]
pub(crate) struct Comparison<'a> {
    pub(crate) id: Uuid,
    pub(crate) images: Vec<Origin<'a>>,
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
        .map(|image| {
            Origin::parse_owned(format!("{}/{}", static_dir.origin, image))
                .expect("BUG: image should be parseable to origin.")
        })
        .collect();

    Ok(Comparison {
        id: *comparison.id,
        images,
    })
}
