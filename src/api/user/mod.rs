pub(crate) mod handler;

use uuid::Uuid;
use serde::Serialize;
use sqlx::SqliteConnection;

use super::SqliteUuid;

#[derive(Serialize)]
pub(crate) struct User {
    pub(crate) id: SqliteUuid,
    pub(crate) comparisons: i64,
    pub(crate) average_lambda: f64,
}

pub(crate) async fn get_user(
    id: Uuid,
    connection: &mut SqliteConnection,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM user WHERE id = ?", id)
        .fetch_one(connection)
        .await
}

pub(crate) async fn generate_user(
    connection: &mut SqliteConnection,
) -> Result<User, sqlx::Error> {
    loop {
        let id = Uuid::new_v4();
        match get_user(id, connection).await {
            Ok(_) => continue,
            Err(sqlx::Error::RowNotFound) => {
                return sqlx::query_as!(
                    User,
                    "INSERT INTO user (id) VALUES (?) RETURNING *",
                    id
                )
                .fetch_one(connection)
                .await
            },
            Err(error) => return Err(error),
        }
    }
}
