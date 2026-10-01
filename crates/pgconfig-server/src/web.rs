//! The web app, embedded in the binary.
//!
//! `web/` holds the sources and `npm run build` renders them into `web/dist`,
//! which is what ships inside the binary. The directory is not committed, so
//! the web build is a step before cargo. A debug build reads the files from
//! disk on every request, which makes `npm run build` enough to see a change.

use axum::body::Body;
use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use rust_embed::RustEmbed;

use crate::v1::{self, Caller};

#[derive(RustEmbed)]
#[folder = "../../web/dist/"]
struct WebAssets;

/// The marker that keeps `web/dist` in the repository. It is not part of the
/// app.
const PLACEHOLDER: &str = ".gitkeep";

/// Answers every request no route claimed.
///
/// A built file is served as it is. Any other read gets `index.html`, because
/// the app routes in the browser and `/guide/mcp` is not a file. Requests under
/// `/v1`, and anything that is not a read, keep the JSON error REST v1 always
/// gave.
pub(crate) async fn serve(caller: Caller) -> Response {
    let reads = caller.method == Method::GET || caller.method == Method::HEAD;
    let path = caller.path().trim_start_matches('/');
    let is_v1 = path
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("v1"))
        && matches!(path.as_bytes().get(2), None | Some(b'/'));
    if !reads || is_v1 {
        return v1::not_found(caller).await;
    }

    if let Some(file) = WebAssets::get(path).filter(|_| path != PLACEHOLDER) {
        // The build names the files under assets/ after their content.
        let cache = if path.starts_with("assets/") {
            "public, max-age=31536000, immutable"
        } else {
            "no-cache"
        };
        return file_response(path, file.data.into_owned(), cache);
    }
    // A missing file under assets/ is a real 404: answering it with the app
    // would hand HTML to a script tag.
    if path.starts_with("assets/") {
        return v1::not_found(caller).await;
    }
    match WebAssets::get("index.html") {
        Some(index) => file_response("index.html", index.data.into_owned(), "no-cache"),
        // The bundle was not built. The API works without it.
        None => v1::not_found(caller).await,
    }
}

fn file_response(path: &str, body: Vec<u8>, cache: &'static str) -> Response {
    let content_type = mime_guess::from_path(path).first_or_octet_stream();
    let mut response = Body::from(body).into_response();
    *response.status_mut() = StatusCode::OK;
    let headers = response.headers_mut();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str(content_type.essence_str())
            .unwrap_or(HeaderValue::from_static("application/octet-stream")),
    );
    headers.insert(CACHE_CONTROL, HeaderValue::from_static(cache));
    response
}
