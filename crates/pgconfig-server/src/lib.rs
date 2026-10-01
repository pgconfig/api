//! The pgconfig HTTP server. One binary serves REST v1, its OpenAPI document,
//! and the MCP endpoint.

mod cors;
mod mcp;
mod v1;

use axum::Router;
use axum::middleware;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// What a deployment can change.
#[derive(Clone, Debug)]
pub struct Config {
    /// The browser origins that may call `/mcp`. A request without an
    /// `Origin`, which is every native client, is always accepted.
    pub mcp_allowed_origins: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mcp_allowed_origins: ["https://pgconfig.org", "https://www.pgconfig.org"]
                .map(String::from)
                .to_vec(),
        }
    }
}

/// The whole application, with the default configuration.
pub fn app() -> Router {
    app_with(Config::default())
}

/// The whole application.
pub fn app_with(config: Config) -> Router {
    let mut openapi = v1::ApiDoc::openapi();
    openapi.info.version = pgconfig::build::TAG.to_string();

    let routes = v1::router()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", openapi))
        .fallback(v1::not_found);

    // The routes sit behind a fallback so the layers below run before any
    // routing: `normalize` rewrites the path the routes match on, and a CORS
    // preflight is answered whatever methods a route accepts.
    let rest = Router::new()
        .fallback_service(routes)
        .layer(middleware::from_fn(cors::allow_any_origin))
        .layer(middleware::map_request(v1::normalize));

    // MCP has its own, stricter origin policy, so it stays outside the
    // any-origin CORS of REST v1.
    mcp::router(config.mcp_allowed_origins)
        .fallback_service(rest)
        .layer(CatchPanicLayer::new())
        .layer(
            // The span carries the method and the path. Both are logged at
            // INFO so every request leaves one line.
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
}
