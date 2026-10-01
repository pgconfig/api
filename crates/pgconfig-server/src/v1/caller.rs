//! What a v1 handler needs to know about the request: the URL the caller
//! used, and the query string.

use std::convert::Infallible;

use axum::extract::FromRequestParts;
use axum::http::header::HOST;
use axum::http::request::Parts;
use axum::http::{HeaderMap, Method};

use super::query::Query;

/// The request target as it arrived, before `normalize` rewrote the path.
#[derive(Clone, Debug)]
pub(crate) struct OriginalTarget(pub String);

pub(crate) struct Caller {
    pub method: Method,
    /// The path and query the caller sent.
    target: String,
    /// The scheme and host the caller used, such as `https://api.pgconfig.org`.
    base_url: String,
}

impl Caller {
    /// The URL of this request, as `links.self` and the conf header show it.
    pub(crate) fn self_link(&self) -> String {
        format!("{}{}", self.base_url, self.target)
    }

    /// The path the caller sent, without the query string.
    pub(crate) fn path(&self) -> &str {
        self.target
            .split_once('?')
            .map_or(self.target.as_str(), |(path, _)| path)
    }

    pub(crate) fn query(&self) -> Query<'_> {
        Query(self.target.split_once('?').map_or("", |(_, query)| query))
    }
}

impl<S: Send + Sync> FromRequestParts<S> for Caller {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Infallible> {
        let target = match parts.extensions.get::<OriginalTarget>() {
            Some(OriginalTarget(target)) => target.clone(),
            None => parts
                .uri
                .path_and_query()
                .map(|t| t.to_string())
                .unwrap_or_default(),
        };
        let host = parts
            .headers
            .get(HOST)
            .and_then(|host| host.to_str().ok())
            .or_else(|| parts.uri.host())
            .unwrap_or_default();
        Ok(Self {
            method: parts.method.clone(),
            target,
            base_url: base_url(&parts.headers, host),
        })
    }
}

/// The scheme and host of the request, taking the forwarded headers a proxy
/// sets over what the connection shows. Fiber trusted these headers from any
/// peer, and production relies on it: the proxy terminates TLS.
fn base_url(headers: &HeaderMap, host: &str) -> String {
    let first = |value: &str| value.split(',').next().unwrap_or_default().to_string();

    let mut scheme = "http".to_string();
    for (name, value) in headers {
        let Ok(value) = value.to_str() else { continue };
        match name.as_str() {
            "x-forwarded-proto" | "x-forwarded-protocol" => scheme = first(value),
            "x-forwarded-ssl" if value == "on" => scheme = "https".to_string(),
            "x-url-scheme" => scheme = value.to_string(),
            _ => {}
        }
    }
    let host = headers
        .get("x-forwarded-host")
        .and_then(|host| host.to_str().ok())
        .filter(|host| !host.is_empty())
        .map_or_else(|| host.to_string(), first);
    format!("{scheme}://{host}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(pairs: &[(&'static str, &'static str)]) -> HeaderMap {
        pairs
            .iter()
            .map(|(name, value)| (name.parse().unwrap(), value.parse().unwrap()))
            .collect()
    }

    #[test]
    fn a_direct_request_is_http_on_its_host() {
        assert_eq!(
            base_url(&HeaderMap::new(), "localhost:3000"),
            "http://localhost:3000"
        );
    }

    #[test]
    fn forwarded_headers_win() {
        let forwarded = headers(&[
            ("x-forwarded-proto", "https, http"),
            ("x-forwarded-host", "api.pgconfig.org, internal"),
        ]);

        assert_eq!(
            base_url(&forwarded, "10.0.0.7:3000"),
            "https://api.pgconfig.org"
        );
    }

    #[test]
    fn forwarded_ssl_only_counts_when_on() {
        assert_eq!(
            base_url(&headers(&[("x-forwarded-ssl", "on")]), "h"),
            "https://h"
        );
        assert_eq!(
            base_url(&headers(&[("x-forwarded-ssl", "off")]), "h"),
            "http://h"
        );
    }
}
