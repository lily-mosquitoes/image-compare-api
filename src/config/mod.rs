use std::sync::Once;

static LOAD_ENV: Once = Once::new();

fn load_environment_once() {
    LOAD_ENV.call_once(|| {
        dotenvy::dotenv().expect(".env to be present");
    });
}

fn load_static_files_dir() -> String {
    load_environment_once();

    std::env::var("STATIC_FILES_DIR")
        .expect("`STATIC_FILES_DIR to be set in .env`")
}

lazy_static! {
    pub(crate) static ref STATIC_FILES_DIR: String =
        load_static_files_dir();
}
