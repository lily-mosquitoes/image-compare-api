use rocket::{
    http::Status,
    outcome::{
        try_outcome,
        IntoOutcome,
    },
    request::{
        FromRequest,
        Outcome,
    },
    serde::json::Json,
    Request,
    State,
};
use rocket_db_pools::Connection;

use super::{
    Admin,
    Comparison,
};
use crate::{
    api::QueryError,
    response::ResponseBody,
    DbPool,
    StaticDir,
};

#[post("/admin/comparison")]
pub(crate) async fn generate_comparisons<'r>(
    static_dir: &State<StaticDir>,
    admin: Admin,
    mut connection: Connection<DbPool>,
) -> (Status, Json<ResponseBody<Vec<Comparison<'r>>, QueryError>>) {
    let comparisons = super::generate_comparisons_from_static_dir(
        &admin,
        static_dir,
        &mut **connection,
    )
    .await;

    match comparisons {
        Err(error) => (error.default_status(), Json(Err(error).into())),
        Ok(comparisons) => (Status::Created, Json(Ok(comparisons).into())),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = ();

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        let mut connection = try_outcome!(request
            .guard::<Connection<DbPool>>()
            .await
            .map_error(|_| (Status::InternalServerError, ())));
        let key = request.headers().get_one("Authorization");

        match key {
            None => Outcome::Forward(Status::Unauthorized),
            Some(key) => {
                let key = key.trim_start_matches("Bearer").trim();
                super::get_admin(key, &mut **connection)
                    .await
                    .ok()
                    .or_forward(Status::Unauthorized)
            },
        }
    }
}
