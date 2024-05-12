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
}

pub(crate) async fn create_or_update_vote(
    vote: &Vote,
    connection: &mut SqliteConnection,
) -> Result<(Status, Vote), QueryError> {
    let _ = super::user::get_user(*vote.user_id, connection).await?;

    let image_found = get_comparison_images(*vote.comparison_id, connection)
        .await?
        .iter()
        .any(|path| *path.path() == vote.image);

    match (
        image_found,
        get_vote(*vote.comparison_id, *vote.user_id, connection).await,
    ) {
        (false, _) => Err(QueryError::RowNotFound(
            "`image` with requested name not found".to_string(),
        )),
        (true, Err(QueryError::RowNotFound(_))) => Ok((
            Status::Created,
            sqlx::query_as!(
                Vote,
                "INSERT INTO vote (comparison_id, user_id, image) VALUES (?, \
                 ?, ?) RETURNING *",
                *vote.comparison_id,
                *vote.user_id,
                vote.image,
            )
            .fetch_one(connection)
            .await?,
        )),
        (true, Err(error)) => Err(error),
        (true, Ok(_)) => Ok((
            Status::Ok,
            sqlx::query_as!(
                Vote,
                "UPDATE vote SET image = ? WHERE comparison_id = ? AND \
                 user_id = ? RETURNING *",
                vote.image,
                *vote.comparison_id,
                *vote.user_id,
            )
            .fetch_one(connection)
            .await?,
        )),
    }
}

async fn get_vote(
    comparison_id: Uuid,
    user_id: Uuid,
    connection: &mut SqliteConnection,
) -> Result<Vote, QueryError> {
    sqlx::query_as!(
        Vote,
        "SELECT * FROM vote WHERE comparison_id = ? AND user_id = ?",
        comparison_id,
        user_id
    )
    .fetch_one(connection)
    .await
    .map_err(|error| match error {
        sqlx::Error::RowNotFound => QueryError::RowNotFound(
            "`vote` with requested comparison_id and user_id not found"
                .to_string(),
        ),
        error => error.into(),
    })
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
