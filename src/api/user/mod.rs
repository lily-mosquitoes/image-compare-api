pub(crate) mod handler;

use rocket::serde::uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct User {
    pub(crate) id: Uuid,
    pub(crate) comparisons: u64,
    pub(crate) average_lambda: f64,
}

pub(crate) fn get_user(id: Uuid) -> Result<User, sqlx::Error> {
    let user = User {
        id,
        comparisons: 7,
        average_lambda: 0.1234,
    };
    Ok(user)
}

pub(crate) fn generate_user() -> Result<User, sqlx::Error> {
    let user = User {
        id: Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap(),
        comparisons: 0,
        average_lambda: 0.0,
    };
    Ok(user)
}
