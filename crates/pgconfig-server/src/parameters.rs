//! `GET /parameters/<version>/<name>.md`: the PostgreSQL manual's entry for
//! one parameter of one major version, as the file in `parameters/` holds it.

use axum::Router;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use pgconfig::{PgVersion, parameter_doc};

pub(crate) fn router() -> Router {
    Router::new().route("/parameters/{version}/{file}", get(serve))
}

async fn serve(Path((version, file)): Path<(String, String)>) -> Response {
    // The address names a major version, such as 18 or 9.6, never a minor
    // release. Parameter names are case-insensitive.
    let doc = file.strip_suffix(".md").and_then(|name| {
        let major = PgVersion::parse(&version).ok()?.major();
        (major.to_string() == version)
            .then(|| parameter_doc(major, name))
            .flatten()
    });
    match doc {
        Some(doc) => (
            [
                (CONTENT_TYPE, "text/markdown; charset=utf-8"),
                (CACHE_CONTROL, "no-cache"),
            ],
            doc.markdown(),
        )
            .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            [(CONTENT_TYPE, "text/plain; charset=utf-8")],
            "No parameter documentation here. Use /parameters/<major version>/<parameter>.md, such as /parameters/18/work_mem.md.",
        )
            .into_response(),
    }
}
