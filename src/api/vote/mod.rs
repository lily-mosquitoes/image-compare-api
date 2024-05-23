pub(crate) mod handler;

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};
use sqlx::SqliteConnection;
use uuid::Uuid;

use super::{
    QueryError,
    SqliteArray,
    SqliteUuid,
};

#[derive(Serialize, Deserialize)]
pub(crate) struct Vote {
    #[serde(skip_deserializing)]
    pub(crate) id: Option<i64>,
    pub(crate) comparison_id: SqliteUuid,
    pub(crate) user_id: SqliteUuid,
    pub(crate) image: String,
    #[serde(skip_deserializing)]
    pub(crate) created_at: DateTime<Utc>,
    #[serde(skip_deserializing)]
    pub(crate) ip_addr: Option<String>,
}

pub(crate) async fn create_vote(
    vote: &Vote,
    connection: &mut SqliteConnection,
) -> Result<Vote, QueryError> {
    let _ = super::user::get_user(*vote.user_id, connection).await?;

    let image_found = get_comparison_images(*vote.comparison_id, connection)
        .await?
        .iter()
        .any(|path| *path.path() == vote.image);

    match image_found {
        false => Err(QueryError::RowNotFound(
            "`image` with requested name not found".to_string(),
        )),
        true => sqlx::query_as!(
            Vote,
            "INSERT INTO vote (comparison_id, user_id, image, ip_addr) VALUES \
             (?1, ?2, ?3, ?4) RETURNING id, comparison_id, user_id, image, \
             created_at as \"created_at: _\", ip_addr",
            *vote.comparison_id,
            *vote.user_id,
            vote.image,
            vote.ip_addr,
        )
        .fetch_one(connection)
        .await
        .map_err(|error| error.into()),
    }
}

async fn get_comparison_images(
    id: Uuid,
    connection: &mut SqliteConnection,
) -> Result<SqliteArray, QueryError> {
    sqlx::query_as!(
        ComparisonImages,
        "SELECT images FROM comparison WHERE id = ?",
        id,
    )
    .fetch_one(connection)
    .await
    .map_err(|error| match error {
        sqlx::Error::RowNotFound => QueryError::RowNotFound(
            "`comparison` with requested id not found".to_string(),
        ),
        error => error.into(),
    })
    .map(|comparison| comparison.images)
}

struct ComparisonImages<'a> {
    images: SqliteArray<'a>,
}
