pub(crate) mod handler;

use std::{
    collections::BTreeMap,
    path::PathBuf,
};

use rocket::{
    http::RawStr,
    State,
};
use sqlx::SqliteConnection;
use uuid::Uuid;

use super::{
    comparison::Comparison,
    QueryError,
};
use crate::StaticDir;

pub(crate) async fn get_admin(
    key: &str,
    connection: &mut SqliteConnection,
) -> Result<Admin, QueryError> {
    sqlx::query_as!(Admin, "SELECT * FROM admin WHERE key = ?", key)
        .fetch_one(connection)
        .await
        .map_err(|error| error.into())
}

pub(crate) struct Admin {
    pub(crate) id: Option<i64>,
    pub(crate) key: String,
}

pub(crate) async fn generate_comparisons_from_static_dir<'r>(
    static_dir: &State<StaticDir>,
    connection: &mut SqliteConnection,
) -> Result<Vec<Comparison<'r>>, QueryError> {
    let static_dir_files = read_dir_files(&static_dir.path, "".to_string())
        .map_err(|error| {
            QueryError::FileServerError(error.kind().to_string())
        })?;

    let mut files_by_dirname: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for path in static_dir_files.into_iter() {
        let mut split = path.rsplitn(2, '/');
        let _filename = split
            .next()
            .expect("BUG: rsplitn should return at least one item");
        let dirname = split.next().unwrap_or("");

        let entry = files_by_dirname.entry(dirname.to_string()).or_default();
        (*entry).push(path);
    }

    let mut comparisons = Vec::new();
    for (dirname, files) in &files_by_dirname {
        if files.len() < 2 {
            return Err(QueryError::FileServerError(format!(
                "Not enough files in STATIC_DIR {dirname} (minimum 2 needed)"
            )));
        }

        for a in 0..(files.len() - 1) {
            for b in (a + 1)..files.len() {
                let comparison_ab = create_comparison(
                    &format!("{}///{}", files[a], files[b]),
                    dirname,
                    connection,
                )
                .await?;
                comparisons.push(comparison_ab);

                let comparison_ba = create_comparison(
                    &format!("{}///{}", files[b], files[a]),
                    dirname,
                    connection,
                )
                .await?;
                comparisons.push(comparison_ba);
            }
        }
    }

    Ok(comparisons)
}

async fn create_comparison<'r>(
    images: &str,
    dirname: &str,
    connection: &mut SqliteConnection,
) -> Result<Comparison<'r>, QueryError> {
    let id = generate_new_comparison_id(connection).await?;

    sqlx::query_as!(
        Comparison,
        "INSERT INTO comparison (id, dirname, images) VALUES (?, ?, ?) ON \
         CONFLICT DO UPDATE SET images=images RETURNING id, dirname, images, \
         created_at as \"created_at: _\"",
        id,
        dirname,
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

fn read_dir_files(
    path: &PathBuf,
    dirname: String,
) -> Result<Vec<String>, std::io::Error> {
    let mut files: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let filename = entry
            .file_name()
            .into_string()
            .map_err(|error| {
                std::io::Error::other(format!(
                    "Invalid UTF-8 character in {error:?}"
                ))
            })
            .map(|filename| {
                RawStr::new(&filename).percent_encode().to_string()
            })?;

        if metadata.is_file() {
            files.push(
                format!("{dirname}/{filename}")
                    .trim_start_matches('/')
                    .to_string(),
            );
        } else if metadata.is_dir() {
            let sub_path = entry.path();

            files.append(&mut read_dir_files(
                &sub_path,
                format!("{dirname}/{filename}")
                    .trim_start_matches('/')
                    .to_string(),
            )?)
        }
    }

    Ok(files)
}

#[cfg(test)]
mod test {
    use std::{
        path::PathBuf,
        str::FromStr,
    };

    use rocket::fs::relative;

    #[test]
    fn read_dir_files_returns_expected_paths() {
        let path = PathBuf::from(relative!(
            "tests/test_static_dirs/with_subfolders_ok"
        ));

        let mut files = super::read_dir_files(&path, "".to_string())
            .expect("Dir to be readable");
        files.sort_unstable();

        let mut expected_files = vec![
            "image%20A.png".to_string(),
            "image%20B.png".to_string(),
            "folder_a/image%201.png".to_string(),
            "folder_a/image%202.png".to_string(),
            "folder_a/image%203.png".to_string(),
            "folder_b/folder_c/image%204.png".to_string(),
            "folder_b/folder_c/image%205.png".to_string(),
        ];
        expected_files.sort_unstable();

        assert_eq!(files, expected_files)
    }
}
