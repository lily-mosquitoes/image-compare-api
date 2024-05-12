pub(crate) mod handler;

use rocket::http::Status;
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
    pub(crate) comparison_id: SqliteUuid,
    pub(crate) user_id: SqliteUuid,
    pub(crate) image: String,
    #[serde(skip)]
    pub(crate) status: QueryStatus,
}

pub(crate) async fn create_or_update_vote(
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
            "INSERT INTO vote (comparison_id, user_id, image, status) VALUES \
             (?1, ?2, ?3, 201) ON CONFLICT DO UPDATE SET image = ?3, status = \
             200 RETURNING *",
            *vote.comparison_id,
            *vote.user_id,
            vote.image,
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

pub(crate) enum QueryStatus {
    Updated,
    Created,
}

impl Default for QueryStatus {
    fn default() -> Self {
        Self::Created
    }
}

impl From<i64> for QueryStatus {
    fn from(value: i64) -> Self {
        match value {
            200 => Self::Updated,
            201 => Self::Created,
            _ => unimplemented!(),
        }
    }
}

impl Vote {
    fn status(&self) -> Status {
        match self.status {
            QueryStatus::Updated => Status::Ok,
            QueryStatus::Created => Status::Created,
        }
    }
}
