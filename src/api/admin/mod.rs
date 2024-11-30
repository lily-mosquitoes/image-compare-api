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
    sqlx::query_as!(Admin, "SELECT id FROM admin WHERE key = ?", key)
        .fetch_one(connection)
        .await
        .map_err(|error| error.into())
}

pub(crate) struct Admin {
    pub(crate) id: i64,
}

pub(crate) async fn generate_comparisons_from_static_dir<'r>(
    admin: &Admin,
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
    for (dirname, files) in &mut files_by_dirname {
        if files.len() < 2 {
            return Err(QueryError::FileServerError(format!(
                "Not enough files in STATIC_DIR/{dirname} (minimum 2 needed)"
            )));
        }

        let truncate_at = get_truncate_at_from_dirname(dirname);

        // assumes the file names are such that
        // default sorting will arrange them
        // by "distance", so "file 1" and "file 2" are
        // more similar and "file 1" and "file 6" are more
        // dissimilar, comparatevely
        files.sort();

        for pair in generate_pairs(files, truncate_at) {
            let comparison = create_comparison(
                &format!("{}///{}", pair.0, pair.1),
                dirname,
                admin,
                connection,
            )
            .await?;
            comparisons.push(comparison);
        }
    }

    Ok(comparisons)
}

fn get_truncate_at_from_dirname(dirname: &String) -> Option<usize> {
    let keyword = "truncate_at_";
    let index = dirname.find(keyword)?;
    let (_, truncate_at) = dirname.split_at(index + keyword.len());
    truncate_at.parse().ok()
}

fn generate_pairs<T: Clone>(
    list: &Vec<T>,
    truncate_at: Option<usize>,
) -> Vec<(T, T)> {
    let mut pairs = Vec::new();
    let truncate_at = truncate_at.unwrap_or(list.len());

    for a in 0..(list.len() - 1) {
        // this truncates the comparisons so we only generate
        // comparisons for "near" elements in the list
        for b in (a + 1)..(a + truncate_at).min(list.len()) {
            pairs.push((list[a].clone(), list[b].clone()));
            pairs.push((list[b].clone(), list[a].clone()));
        }
    }

    pairs
}

async fn create_comparison<'r>(
    images: &str,
    dirname: &str,
    admin: &Admin,
    connection: &mut SqliteConnection,
) -> Result<Comparison<'r>, QueryError> {
    let id = generate_new_comparison_id(connection).await?;

    sqlx::query_as!(
        Comparison,
        "INSERT INTO comparison (id, dirname, images, created_by) VALUES (?, \
         ?, ?, ?) ON CONFLICT DO UPDATE SET images=images RETURNING id, \
         dirname, images, created_at as \"created_at: _\", created_by",
        id,
        dirname,
        images,
        admin.id
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
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;
    use rocket::fs::relative;

    #[test]
    fn read_dir_files_returns_expected_paths() {
        let path = PathBuf::from(relative!("tests/static_dir/ok"));

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
            "folder_d_truncate_at_2/image%201.png".to_string(),
            "folder_d_truncate_at_2/image%202.png".to_string(),
            "folder_d_truncate_at_2/image%203.png".to_string(),
            "folder_d_truncate_at_2/image%204.png".to_string(),
        ];
        expected_files.sort_unstable();

        assert_eq!(files, expected_files)
    }

    #[test]
    fn generate_pairs_returns_all_permutations() {
        let test_list = vec![
            "image%201.png".to_string(),
            "image%202.png".to_string(),
            "image%203.png".to_string(),
            "image%204.png".to_string(),
            "image%205.png".to_string(),
            "image%206.png".to_string(),
            "image%207.png".to_string(),
            "image%208.png".to_string(),
            "image%209.png".to_string(),
            "image%2010.png".to_string(),
            "image%2011.png".to_string(),
            "image%2012.png".to_string(),
            "image%2013.png".to_string(),
            "image%2014.png".to_string(),
            "image%2015.png".to_string(),
            "image%2016.png".to_string(),
            "image%2017.png".to_string(),
            "image%2018.png".to_string(),
            "image%2019.png".to_string(),
            "image%2020.png".to_string(),
            "image%2021.png".to_string(),
            "image%2022.png".to_string(),
        ];

        // permutations = n! / (n-r)!
        let expected_permutations_len = 462;

        let test_permutations = super::generate_pairs(&test_list, None);

        assert_eq!(test_permutations.len(), expected_permutations_len);
    }

    #[test]
    fn generate_pairs_with_truncated_at_returns_limited_permutations() {
        let test_list = vec![
            "image%201.png".to_string(),
            "image%202.png".to_string(),
            "image%203.png".to_string(),
            "image%204.png".to_string(),
            "image%205.png".to_string(),
            "image%206.png".to_string(),
            "image%207.png".to_string(),
            "image%208.png".to_string(),
        ];

        let expected_permutations_truncated = vec![
            ("image%201.png".to_string(), "image%202.png".to_string()),
            ("image%202.png".to_string(), "image%201.png".to_string()),
            ("image%201.png".to_string(), "image%203.png".to_string()),
            ("image%203.png".to_string(), "image%201.png".to_string()),
            //
            ("image%202.png".to_string(), "image%203.png".to_string()),
            ("image%203.png".to_string(), "image%202.png".to_string()),
            ("image%202.png".to_string(), "image%204.png".to_string()),
            ("image%204.png".to_string(), "image%202.png".to_string()),
            //
            ("image%203.png".to_string(), "image%204.png".to_string()),
            ("image%204.png".to_string(), "image%203.png".to_string()),
            ("image%203.png".to_string(), "image%205.png".to_string()),
            ("image%205.png".to_string(), "image%203.png".to_string()),
            //
            ("image%204.png".to_string(), "image%205.png".to_string()),
            ("image%205.png".to_string(), "image%204.png".to_string()),
            ("image%204.png".to_string(), "image%206.png".to_string()),
            ("image%206.png".to_string(), "image%204.png".to_string()),
            //
            ("image%205.png".to_string(), "image%206.png".to_string()),
            ("image%206.png".to_string(), "image%205.png".to_string()),
            ("image%205.png".to_string(), "image%207.png".to_string()),
            ("image%207.png".to_string(), "image%205.png".to_string()),
            //
            ("image%206.png".to_string(), "image%207.png".to_string()),
            ("image%207.png".to_string(), "image%206.png".to_string()),
            ("image%206.png".to_string(), "image%208.png".to_string()),
            ("image%208.png".to_string(), "image%206.png".to_string()),
            //
            ("image%207.png".to_string(), "image%208.png".to_string()),
            ("image%208.png".to_string(), "image%207.png".to_string()),
        ];

        let test_permutations_truncated =
            super::generate_pairs(&test_list, Some(3));

        assert_eq!(
            test_permutations_truncated,
            expected_permutations_truncated
        );
    }

    #[test]
    fn get_truncate_at_from_dirname() {
        let tests = vec![
            ("test_dir_truncate_at_42".to_string(), Some(42)),
            ("test_dir_truncate_at_42_blah".to_string(), None),
            ("test_dir_truncate_at_blah".to_string(), None),
            ("test_dir".to_string(), None),
        ];

        for (test, expected_truncate_at) in tests {
            let truncate_at = super::get_truncate_at_from_dirname(&test);

            assert_eq!(truncate_at, expected_truncate_at);
        }
    }
}
