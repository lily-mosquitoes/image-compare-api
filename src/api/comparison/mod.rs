pub(crate) mod handler;

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
    pub(crate) images: SqliteArray<'a>,
}

async fn get_comparison_for_user<'r>(
    user_id: Uuid,
    connection: &mut SqliteConnection,
) -> Result<Comparison<'r>, QueryError> {
    sqlx::query_as!(
        Comparison,
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
}
