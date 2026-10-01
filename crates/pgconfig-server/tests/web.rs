//! The web app the server embeds: its files, and the fallback that lets the
//! app's own router handle every address the server does not know.
//!
//! These tests need the bundle. Run `npm ci && npm run build` in `web/` first.

use axum::body::Body;
use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use axum::http::{HeaderMap, Method, Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn request(method: Method, path: &str) -> (StatusCode, HeaderMap, String) {
    let request = Request::builder()
        .method(method)
        .uri(path)
        .body(Body::empty())
        .unwrap();
    let response = pgconfig_server::app().oneshot(request).await.unwrap();
    let (parts, body) = response.into_parts();
    let body = body.collect().await.unwrap().to_bytes();
    (
        parts.status,
        parts.headers,
        String::from_utf8_lossy(&body).into_owned(),
    )
}

async fn get(path: &str) -> (StatusCode, HeaderMap, String) {
    request(Method::GET, path).await
}

fn is_the_app(body: &str) -> bool {
    body.contains("<div id=\"root\">")
}

#[tokio::test]
async fn the_root_serves_the_app() {
    let (status, headers, body) = get("/").await;

    assert_eq!(status, StatusCode::OK);
    assert!(
        is_the_app(&body),
        "the web bundle is missing: run `npm ci && npm run build` in web/"
    );
    assert_eq!(headers[CONTENT_TYPE], "text/html");
    assert_eq!(headers[CACHE_CONTROL], "no-cache");
}

#[tokio::test]
async fn an_address_of_the_app_falls_back_to_the_app() {
    for path in [
        "/guide",
        "/guide/mcp",
        "/export?pg_version=17",
        "/tuning",
        "/no-such-page",
    ] {
        let (status, _, body) = get(path).await;

        assert_eq!(status, StatusCode::OK, "{path}");
        assert!(is_the_app(&body), "{path}");
    }
}

#[tokio::test]
async fn every_built_file_is_served_with_its_type() {
    let (_, _, index) = get("/").await;
    let assets: Vec<&str> = index
        .split('"')
        .filter(|part| part.starts_with("/assets/"))
        .collect();
    assert!(assets.iter().any(|asset| asset.ends_with(".js")), "{index}");
    assert!(
        assets.iter().any(|asset| asset.ends_with(".css")),
        "{index}"
    );

    for asset in assets {
        let (status, headers, body) = get(asset).await;

        assert_eq!(status, StatusCode::OK, "{asset}");
        assert!(!is_the_app(&body), "{asset} fell back to the app");
        let expected = if asset.ends_with(".js") {
            "text/javascript"
        } else {
            "text/css"
        };
        assert_eq!(headers[CONTENT_TYPE], expected, "{asset}");
        // The build names each file after its content, so it never changes.
        assert_eq!(
            headers[CACHE_CONTROL], "public, max-age=31536000, immutable",
            "{asset}"
        );
    }
}

#[tokio::test]
async fn a_file_outside_assets_is_served_without_the_long_cache() {
    let (status, headers, _) = get("/pgconfig.svg").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers[CONTENT_TYPE], "image/svg+xml");
    assert_eq!(headers[CACHE_CONTROL], "no-cache");
}

#[tokio::test]
async fn a_missing_asset_is_not_the_app() {
    let (status, _, body) = get("/assets/gone-1234.js").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(!is_the_app(&body));
}

#[tokio::test]
async fn an_unknown_v1_route_is_still_a_json_error() {
    let (status, headers, body) = get("/v1/nope").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(headers[CONTENT_TYPE], "application/json");
    assert!(body.contains("Cannot GET /v1/nope"), "{body}");
}

#[tokio::test]
async fn only_a_read_falls_back_to_the_app() {
    let (status, _, body) = request(Method::POST, "/guide").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("Cannot POST /guide"), "{body}");
}

#[tokio::test]
async fn the_swagger_ui_keeps_its_address() {
    let (status, _, body) = get("/docs/").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("swagger-ui"));
    assert!(!is_the_app(&body));
}
