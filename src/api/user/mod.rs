pub(crate) mod handler;

use rocket::serde::uuid::Uuid;
use rocket_db_pools::Connection;
use serde::Serialize;

use crate::DbPool;

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
    mut connection: Connection<DbPool>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM user WHERE id = ?", id)
        .fetch_one(&mut **connection)
        .await
}

pub(crate) fn generate_user() -> Result<User, sqlx::Error> {
    let user = User {
        id: SqliteUuid(
            Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6")
                .unwrap()
                .into_bytes()
                .to_vec(),
        ),
        comparisons: 0,
        average_lambda: 0.0,
    };
    Ok(user)
}
