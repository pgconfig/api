//! The pgconfig HTTP server. One binary serves REST v1 and its OpenAPI
//! document.

mod cors;
mod v1;

use axum::Router;
use axum::middleware;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// The whole application.
pub fn app() -> Router {
    let mut openapi = v1::ApiDoc::openapi();
    openapi.info.version = pgconfig::build::TAG.to_string();

    let routes = v1::router()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", openapi))
        .fallback(v1::not_found);

    // The routes sit behind a fallback so the layers below run before any
    // routing: `normalize` rewrites the path the routes match on, and a CORS
    // preflight is answered whatever methods a route accepts.
    Router::new()
        .fallback_service(routes)
        .layer(middleware::from_fn(cors::allow_any_origin))
        .layer(middleware::map_request(v1::normalize))
        .layer(CatchPanicLayer::new())
        .layer(
            // The span carries the method and the path. Both are logged at
            // INFO so every request leaves one line.
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
}
