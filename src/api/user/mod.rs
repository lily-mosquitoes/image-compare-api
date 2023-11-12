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
