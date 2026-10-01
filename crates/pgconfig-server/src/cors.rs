//! Cross-origin access for REST v1, as the Go API granted it: any origin may
//! read.

use axum::body::Body;
use axum::extract::Request;
use axum::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    ACCESS_CONTROL_REQUEST_HEADERS, ACCESS_CONTROL_REQUEST_METHOD, ORIGIN, VARY,
};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

/// A request without an `Origin` is not a cross-origin request and passes
/// through untouched. A preflight is answered here. Any other request gets
/// `Access-Control-Allow-Origin: *` on its response.
pub(crate) async fn allow_any_origin(request: Request, next: Next) -> Response {
    let headers = request.headers();
    if !headers.contains_key(ORIGIN) {
        return next.run(request).await;
    }

    let preflight =
        request.method() == Method::OPTIONS && headers.contains_key(ACCESS_CONTROL_REQUEST_METHOD);
    let mut response = if preflight {
        let mut response = Response::new(Body::empty());
        *response.status_mut() = StatusCode::NO_CONTENT;
        let out = response.headers_mut();
        out.insert(
            VARY,
            HeaderValue::from_static(
                "Access-Control-Request-Method, Access-Control-Request-Headers, Origin",
            ),
        );
        out.insert(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET,POST,HEAD,PUT,DELETE,PATCH"),
        );
        if let Some(requested) = headers.get(ACCESS_CONTROL_REQUEST_HEADERS) {
            out.insert(ACCESS_CONTROL_ALLOW_HEADERS, requested.clone());
        }
        response
    } else {
        next.run(request).await
    };
    response
        .headers_mut()
        .insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
}
