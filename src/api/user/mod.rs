pub(crate) mod handler;

use rocket::serde::uuid::Uuid;
use serde::Serialize;
use sqlx::SqliteConnection;

#[derive(Clone, Serialize)]
#[serde(into = "Uuid")]
pub(crate) struct SqliteUuid(Vec<u8>);

impl From<Vec<u8>> for SqliteUuid {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<SqliteUuid> for Uuid {
    fn from(value: SqliteUuid) -> Self {
        let mut bytes = value.0.clone();
        bytes.truncate(16);

        Self::from_slice(&bytes)
            .expect("BUG: 16 byte array should be a valid UUID.")
    }
}

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
