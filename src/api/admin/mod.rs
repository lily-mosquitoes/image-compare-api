use rocket::State;
use sqlx::SqliteConnection;
use uuid::Uuid;

use super::{
    comparison::Comparison,
    QueryError,
};
use crate::StaticDir;

pub(crate) async fn generate_comparisons_from_static_dir<'r>(
    static_dir: &State<StaticDir>,
    connection: &mut SqliteConnection,
) -> Result<Vec<Comparison<'r>>, QueryError> {
    let images = read_dir_entries(static_dir).map_err(|error| {
        QueryError::FileServerError(error.kind().to_string())
    })?;

    if images.len() < 2 {
        return Err(QueryError::FileServerError(
            "Not enough files in STATIC_DIR (minimum 2 needed)".to_string(),
        ));
    }

    let mut comparisons = Vec::new();
    for a in 0..(images.len() - 1) {
        for b in 1..images.len() {
            let comparison_ab = create_comparison(
                format!("{}/{}", images[a], images[b]),
                connection,
            )
            .await?;
            comparisons.push(comparison_ab);

            let comparison_ba = create_comparison(
                format!("{}/{}", images[b], images[a]),
                connection,
            )
            .await?;
            comparisons.push(comparison_ba);
        }
    }

    Ok(comparisons)
}

async fn create_comparison<'r>(
    images: String,
    connection: &mut SqliteConnection,
) -> Result<Comparison<'r>, QueryError> {
    let id = generate_new_comparison_id(connection).await?;

    sqlx::query_as!(
        Comparison,
        "INSERT INTO comparison (id, images) VALUES (?, ?) ON CONFLICT DO \
         UPDATE SET images=images RETURNING *",
        id,
        images
    )
    .fetch_one(connection)
    .await
    .map_err(|error| error.into())
}

/// Generates a UUID v4 and checks the database table to guarantee no
/// collisions, loops until a suitable UUID is generated.
/// May return other DB errors, like connection errors.
async fn generate_new_comparison_id(
    connection: &mut SqliteConnection,
) -> Result<Uuid, QueryError> {
    loop {
        let id = Uuid::new_v4();
        let result = sqlx::query!("SELECT id FROM comparison WHERE id = ?", id)
            .fetch_one(&mut *connection)
            .await;

        match result {
            Ok(_) => continue,
            Err(sqlx::Error::RowNotFound) => return Ok(id),
            Err(error) => return Err(error.into()),
        }
    }
}

fn read_dir_entries(
    static_dir: &State<StaticDir>,
) -> Result<Vec<String>, std::io::Error> {
    std::fs::read_dir(&static_dir.path)?
        .map(|entry| {
            entry?.file_name().into_string().map_err(|error| {
                std::io::Error::other(format!(
                    "Invalid UTF-8 character in {:?}",
                    error
                ))
            })
        })
        .collect()
}
