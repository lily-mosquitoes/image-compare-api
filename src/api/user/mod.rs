pub(crate) mod handler;

use serde::Serialize;
use sqlx::SqliteConnection;
use uuid::Uuid;

use super::{
    QueryError,
    SqliteUuid,
};

#[derive(Serialize)]
pub(crate) struct User {
    pub(crate) id: SqliteUuid,
    pub(crate) votes: i64,
    pub(crate) average_lambda: f64,
}

pub(crate) async fn get_user(
    id: Uuid,
    connection: &mut SqliteConnection,
) -> Result<User, QueryError> {
    sqlx::query_as!(
        User,
        "SELECT user.*, (SELECT COUNT(vote.id) FROM vote WHERE vote.user_id = \
         user.id) as `votes!: i64` FROM user WHERE user.id = ?",
        id
    )
    .fetch_one(connection)
    .await
    .map_err(|error| match error {
        sqlx::Error::RowNotFound => QueryError::RowNotFound(
            "`user` with requested id not found".to_string(),
        ),
        error => error.into(),
    })
}

pub(crate) async fn generate_user(
    connection: &mut SqliteConnection,
) -> Result<User, QueryError> {
    loop {
        let id = Uuid::new_v4();
        match get_user(id, connection).await {
            Ok(_) => continue,
            Err(QueryError::RowNotFound(_)) => {
                return sqlx::query_as!(
                    User,
                    "INSERT INTO user (id) VALUES (?) RETURNING *, 0 as votes",
                    id
                )
                .fetch_one(connection)
                .await
                .map_err(|error| error.into())
            },
            Err(error) => return Err(error),
        }
    }
}
