pub(crate) mod handler;

use rocket::{
    http::uri::Origin,
    State,
};
use serde::Serialize;
use sqlx::SqliteConnection;
use uuid::Uuid;

use super::{
    QueryError,
    SqliteArray,
    SqliteUuid,
};
use crate::StaticDir;

#[derive(Debug, Serialize)]
pub(crate) struct Comparison<'a> {
    pub(crate) id: Uuid,
    pub(crate) images: Vec<Origin<'a>>,
}

async fn get_comparison_for_user<'r>(
    user_id: Uuid,
    connection: &mut SqliteConnection,
    static_dir: &State<StaticDir>,
) -> Result<Comparison<'r>, QueryError> {
    sqlx::query_as!(
        ComparisonRaw,
        "SELECT * FROM comparison WHERE id NOT IN (SELECT comparison_id FROM \
         vote WHERE user_id = ?) LIMIT 1",
        user_id
    )
    .fetch_one(connection)
    .await
    .map_err(|error| match error {
        sqlx::Error::RowNotFound => QueryError::RowNotFound(
            "No `comparison` available for `user`".to_string(),
        ),
        error => error.into(),
    })
    .map(|comparison_raw| Comparison::parse_with(comparison_raw, static_dir))
}

struct ComparisonRaw {
    id: SqliteUuid,
    images: SqliteArray,
}

impl<'r> Comparison<'r> {
    fn parse_with(
        comparison_raw: ComparisonRaw,
        static_dir: &State<StaticDir>,
    ) -> Comparison<'r> {
        let images = comparison_raw
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

        Comparison {
            id: *comparison_raw.id,
            images,
        }
    }
}
