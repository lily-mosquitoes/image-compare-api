pub(crate) mod handler;

use std::fmt::Display;

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
    pub(crate) vote_value: VoteValue,
    #[serde(skip_deserializing)]
    pub(crate) created_at: DateTime<Utc>,
    #[serde(skip_deserializing)]
    pub(crate) ip_addr: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum VoteValue {
    #[serde(rename = "equal")]
    Equal,
    #[serde(rename = "different")]
    Different,
    #[serde(untagged)]
    OneIsBetter(String),
}

pub(crate) async fn create_vote(
    vote: &Vote,
    connection: &mut SqliteConnection,
) -> Result<Vote, QueryError> {
    let _ = super::user::get_user(*vote.user_id, connection).await?;

    if let VoteValue::OneIsBetter(image) = &vote.vote_value {
        get_comparison_images(*vote.comparison_id, connection)
            .await?
            .iter()
            .any(|path| *path.path() == *image)
            .then_some(())
            .ok_or(QueryError::RowNotFound(
                "`image` not found for requested `comparison`".to_string(),
            ))?;
    }

    let vote_value_as_str = vote.vote_value.to_string();

    sqlx::query_as!(
        Vote,
        "INSERT INTO vote (comparison_id, user_id, vote_value, ip_addr) \
         VALUES (?1, ?2, ?3, ?4) RETURNING id, comparison_id, user_id, \
         vote_value, created_at as \"created_at: _\", ip_addr",
        *vote.comparison_id,
        *vote.user_id,
        vote_value_as_str,
        vote.ip_addr,
    )
    .fetch_one(connection)
    .await
    .map_err(|error| error.into())
}

impl From<String> for VoteValue {
    fn from(value: String) -> Self {
        match value.as_str() {
            "equal" => VoteValue::Equal,
            "different" => VoteValue::Different,
            _ => VoteValue::OneIsBetter(value),
        }
    }
}

impl Display for VoteValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            VoteValue::Equal => &"equal".to_string(),
            VoteValue::Different => &"different".to_string(),
            VoteValue::OneIsBetter(s) => s,
        };
        write!(f, "{}", s)
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
