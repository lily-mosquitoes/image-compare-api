pub(crate) mod handler;

use chrono::{
    DateTime,
    Utc,
};
use serde::Serialize;
use sqlx::SqliteConnection;
use uuid::Uuid;

use super::{
    QueryError,
    SqliteArray,
    SqliteUuid,
};

#[derive(Serialize)]
pub(crate) struct Comparison<'a> {
    pub(crate) id: SqliteUuid,
    pub(crate) dirname: String,
    pub(crate) images: SqliteArray<'a>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) created_by: i64,
}

async fn get_comparison_for_user<'r>(
    user_id: Uuid,
    dirname: String,
    connection: &mut SqliteConnection,
) -> Result<Comparison<'r>, QueryError> {
    sqlx::query_as!(
        Comparison,
        "SELECT id, dirname, images, created_at as \"created_at: _\", \
         created_by FROM comparison WHERE comparison.dirname = ?1 AND \
         comparison.id NOT IN (SELECT comparison_id FROM vote WHERE user_id = \
         ?2) ORDER BY RANDOM() LIMIT 1",
        dirname,
        user_id,
    )
    .fetch_one(connection)
    .await
    .map_err(|error| match error {
        sqlx::Error::RowNotFound => QueryError::RowNotFound(
            "No `comparison` available for `user`".to_string(),
        ),
        error => error.into(),
    })
}

async fn get_comparison_dirnames(
    connection: &mut SqliteConnection,
) -> Result<Vec<String>, QueryError> {
    sqlx::query!("SELECT DISTINCT dirname FROM comparison")
        .fetch_all(connection)
        .await
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => QueryError::RowNotFound(
                "No `comparison`s available".to_string(),
            ),
            error => error.into(),
        })
        .map(|results| {
            results.iter().map(|entry| entry.dirname.clone()).collect()
        })
}
