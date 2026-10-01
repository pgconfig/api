//! The OpenAPI document and the Swagger UI under `/docs`.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

async fn get(path: &str) -> (StatusCode, String) {
    let request = Request::builder().uri(path).body(Body::empty()).unwrap();
    let response = pgconfig_server::app().oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8(body.to_vec()).unwrap())
}

#[tokio::test]
async fn the_openapi_document_describes_the_four_v1_routes() {
    let (status, body) = get("/docs/openapi.json").await;
    let document: Value = serde_json::from_str(&body).unwrap();

    assert_eq!(status, StatusCode::OK);
    let mut paths: Vec<&String> = document["paths"].as_object().unwrap().keys().collect();
    paths.sort();
    assert_eq!(
        paths,
        [
            "/v1/tuning/get-config",
            "/v1/tuning/get-config-all-environments",
            "/v1/tuning/list-environments",
            "/v1/version",
        ]
    );
}

#[tokio::test]
async fn the_openapi_document_states_the_real_defaults() {
    let (_, body) = get("/docs/openapi.json").await;
    let document: Value = serde_json::from_str(&body).unwrap();

    let parameters = document["paths"]["/v1/tuning/get-config"]["get"]["parameters"]
        .as_array()
        .unwrap();
    let default = |name: &str| {
        parameters
            .iter()
            .find(|parameter| parameter["name"] == name)
            .map(|parameter| parameter["schema"]["default"].clone())
            .unwrap()
    };
    assert_eq!(default("pg_version"), "18");
    assert_eq!(default("drive_type"), "HDD");
    assert_eq!(default("max_connections"), 100);
}

#[tokio::test]
async fn docs_serves_the_swagger_ui() {
    let (status, body) = get("/docs/").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("swagger-ui"), "{body}");
}

#[tokio::test]
async fn the_old_swagger_address_still_opens_the_ui() {
    let (status, body) = get("/docs/index.html").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("swagger-ui"), "{body}");
}
