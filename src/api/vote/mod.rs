pub(crate) mod handler;

use serde::{
    Deserialize,
    Serialize,
};
use sqlx::SqliteConnection;
use uuid::Uuid;

use super::{
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
) -> Result<(bool, Vote), sqlx::Error> {
    let _ = super::user::get_user(*vote.user_id, connection).await?;

    let image = vote
        .image
        .split("/")
        .last()
        .expect("BUG: String split should return at least one item");

    let image_is_valid = get_comparison_images(*vote.comparison_id, connection)
        .await?
        .iter()
        .any(|image_filename| image_filename == image);

    match (
        image_is_valid,
        get_vote(*vote.comparison_id, *vote.user_id, connection).await,
    ) {
        (true, Ok(_)) => Ok((
            true,
            sqlx::query_as!(
                Vote,
                "UPDATE vote SET image = ? WHERE comparison_id = ? AND \
                 user_id = ? RETURNING *",
                image,
                *vote.comparison_id,
                *vote.user_id,
            )
            .fetch_one(connection)
            .await?,
        )),
        (true, Err(sqlx::Error::RowNotFound)) => Ok((
            false,
            sqlx::query_as!(
                Vote,
                "INSERT INTO vote (comparison_id, user_id, image) VALUES (?, \
                 ?, ?) RETURNING *",
                *vote.comparison_id,
                *vote.user_id,
                image,
            )
            .fetch_one(connection)
            .await?,
        )),
        (true, Err(error)) => Err(error),
        (false, _) => Err(sqlx::Error::RowNotFound),
    }
}

async fn get_vote(
    comparison_id: Uuid,
    user_id: Uuid,
    connection: &mut SqliteConnection,
) -> Result<Vote, sqlx::Error> {
    sqlx::query_as!(
        Vote,
        "SELECT * FROM vote WHERE comparison_id = ? AND user_id = ?",
        comparison_id,
        user_id
    )
    .fetch_one(connection)
    .await
}

async fn get_comparison_images(
    id: Uuid,
    connection: &mut SqliteConnection,
) -> Result<SqliteArray, sqlx::Error> {
    let result = sqlx::query_as!(
        ComparisonImages,
        "SELECT images FROM comparison WHERE id = ?",
        id,
    )
    .fetch_one(connection)
    .await?;

    Ok(result.images)
}

struct ComparisonImages {
    images: SqliteArray,
}
