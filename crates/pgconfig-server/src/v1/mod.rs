//! REST v1, served the way the Go API served it.
//!
//! Installers call these routes unattended, so nothing here may change: the
//! defaults, the response envelope, the error texts, and the HTTP 500 that
//! every invalid input gets. The goldens in `tests/golden/rest` pin all of it.

mod caller;
mod query;

use std::collections::BTreeMap;

use axum::Json;
use axum::Router;
use axum::extract::Request;
use axum::http::header::{ALLOW, CACHE_CONTROL, CONTENT_TYPE};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use pgconfig::{Profile, build, v1};
use serde::Serialize;
use serde_json::{Value, json};
use utoipa::{IntoParams, ToSchema};

pub(crate) use caller::{Caller, OriginalTarget};
use query::Query;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/v1/version", get(version))
        .route("/v1/tuning/list-environments", get(list_environments))
        .route("/v1/tuning/get-config", get(get_config))
        .route(
            "/v1/tuning/get-config-all-environments",
            get(get_config_all_environments),
        )
        .method_not_allowed_fallback(method_not_allowed)
}

/// Fiber matched routes without regard to case or to one trailing slash. This
/// rewrites a `/v1` path to the form the router knows and keeps the target
/// the caller sent, which `links.self` echoes.
pub(crate) async fn normalize(mut request: Request) -> Request {
    let target = request
        .uri()
        .path_and_query()
        .map(|target| target.to_string())
        .unwrap_or_default();
    let path = request.uri().path();
    let is_v1 = path
        .get(..3)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("/v1"));
    if is_v1 {
        let mut normalized = path.to_ascii_lowercase();
        if normalized.len() > 1 && normalized.ends_with('/') {
            normalized.pop();
        }
        if let Some(query) = request.uri().query() {
            normalized = format!("{normalized}?{query}");
        }
        if let Ok(uri) = normalized.parse::<Uri>() {
            *request.uri_mut() = uri;
        }
    }
    request.extensions_mut().insert(OriginalTarget(target));
    request
}

/// The envelope of every successful v1 response.
#[derive(Serialize, ToSchema)]
pub(crate) struct ResponseHttp {
    #[schema(value_type = Object)]
    data: Value,
    jsonapi: JsonApi,
    links: Links,
    meta: Meta,
}

#[derive(Serialize, ToSchema)]
struct JsonApi {
    version: &'static str,
}

#[derive(Serialize, ToSchema)]
struct Links {
    #[serde(rename = "self")]
    self_link: String,
}

#[derive(Serialize, ToSchema)]
struct Meta {
    /// The query arguments as received.
    arguments: BTreeMap<String, Vec<String>>,
    copyright: &'static str,
    version: String,
}

const JSON_API: JsonApi = JsonApi { version: "1.0" };

fn success(caller: &Caller, data: impl Serialize) -> Response {
    Json(ResponseHttp {
        data: serde_json::to_value(data).expect("v1 data is plain JSON"),
        jsonapi: JSON_API,
        links: Links {
            self_link: caller.self_link(),
        },
        meta: Meta {
            arguments: caller.query().arguments(),
            copyright: "PGConfig API",
            version: build::pretty(),
        },
    })
    .into_response()
}

fn failure(caller: &Caller, status: StatusCode, message: &str) -> Response {
    let body = json!({
        "errors": {"code": status.as_u16(), "message": message},
        "links": {"self": caller.self_link()},
        "jsonapi": JSON_API,
    });
    (status, Json(body)).into_response()
}

/// An unknown route under any path.
pub(crate) async fn not_found(caller: Caller) -> Response {
    let message = format!("Cannot {} {}", caller.method, caller.path());
    failure(&caller, StatusCode::NOT_FOUND, &message)
}

async fn method_not_allowed(caller: Caller) -> Response {
    let mut response = failure(
        &caller,
        StatusCode::METHOD_NOT_ALLOWED,
        "Method Not Allowed",
    );
    response
        .headers_mut()
        .insert(ALLOW, "GET, HEAD".parse().expect("a header value"));
    response
}

/// The query parameters of the tuning routes. The handlers read the raw query
/// string themselves, so this only feeds the OpenAPI document.
#[derive(IntoParams)]
#[into_params(parameter_in = Query)]
#[allow(dead_code)]
struct ConfigParams {
    /// PostgreSQL version, from 9.1 to 19. Version 19 is beta; the default remains 18.
    #[param(default = "18")]
    pg_version: Option<String>,
    /// Total memory dedicated to PostgreSQL, such as 8GB.
    #[param(default = "2GB")]
    total_ram: Option<String>,
    /// Total expected number of connections.
    #[param(default = 100)]
    max_connections: Option<i64>,
    /// Application profile of the server: WEB, OLTP, DW, MIXED, or DESKTOP.
    /// Ignored by get-config-all-environments.
    #[param(default = "WEB")]
    environment_name: Option<String>,
    /// Operating system: linux, windows, unix, or darwin.
    #[param(default = "linux")]
    os_type: Option<String>,
    /// Server architecture: 386, i686, amd64, x86-64, arm, or arm64.
    #[param(default = "amd64")]
    arch: Option<String>,
    /// Storage type: HDD, SSD, or SAN.
    #[param(default = "HDD")]
    drive_type: Option<String>,
    /// Total logical CPUs available.
    #[param(default = 2)]
    cpus: Option<i64>,
    /// Output format: json, conf, alter_system, or stackgres. Ignored by
    /// get-config-all-environments, which always answers JSON.
    #[param(default = "json")]
    format: Option<String>,
    /// Set to true to attach the documentation of each parameter.
    #[param(default = "false")]
    show_doc: Option<String>,
    /// Set to true to add the pgbadger logging configuration. Ignored by
    /// get-config-all-environments.
    #[param(default = "false")]
    include_pgbadger: Option<String>,
    /// Log format for pgbadger: stderr, csvlog, syslog, or jsonlog. The
    /// default is jsonlog from PostgreSQL 15 and stderr before it.
    log_format: Option<String>,
}

/// One tuning call, parsed with the defaults of the Go API.
struct Call {
    input: v1::Input,
    format: String,
    show_doc: bool,
    pgbadger: Option<String>,
}

impl Call {
    fn parse(query: Query<'_>) -> Result<Self, String> {
        let get = |key: &str, default: &str| query.get(key).unwrap_or_else(|| default.to_string());
        let pg_version = v1::parse_pg_version(&get("pg_version", "18"))
            .map_err(|error| format!("could not parse pg version: {error}"))?;
        let max_connections = v1::parse_int(&get("max_connections", "100"))
            .map_err(|error| format!("could not parse max connections: {error}"))?;
        let total_cpu = v1::parse_int(&get("cpus", "2"))
            .map_err(|error| format!("could not parse cpus: {error}"))?;
        let total_ram = v1::parse_bytes(&get("total_ram", "2GB"));
        let profile = v1::parse_profile(&get("environment_name", "WEB"))
            .map_err(|error| format!("could not parse environment name: {error}"))?;

        // PostgreSQL 15 added jsonlog.
        let default_log_format = if pg_version >= 15.0 {
            "jsonlog"
        } else {
            "stderr"
        };
        Ok(Self {
            input: v1::Input {
                pg_version,
                total_ram,
                total_cpu,
                max_connections,
                profile,
                os: get("os_type", "linux"),
                arch: get("arch", "amd64"),
                drive_type: get("drive_type", "HDD"),
            },
            format: get("format", "json"),
            show_doc: get("show_doc", "false") == "true",
            pgbadger: (get("include_pgbadger", "false") == "true")
                .then(|| get("log_format", default_log_format)),
        })
    }

    fn categories(&self, pgbadger: Option<&str>) -> Result<Vec<v1::Category>, String> {
        let mut categories = v1::categories(&self.input, pgbadger)
            .map_err(|error| format!("could not process rule: {error}"))?;
        if self.show_doc {
            v1::add_documentation(&mut categories, self.input.pg_version);
        }
        Ok(categories)
    }
}

/// v1 answers every invalid input with HTTP 500. That is wrong, and it is the
/// contract.
fn invalid(caller: &Caller, stage: &str, error: &str) -> Response {
    failure(
        caller,
        StatusCode::INTERNAL_SERVER_ERROR,
        &format!("{stage}: {error}"),
    )
}

/// Get Configuration
///
/// Computes the input and suggests a tuning configuration.
#[utoipa::path(
    get,
    path = "/v1/tuning/get-config",
    tag = "tuning",
    params(ConfigParams),
    responses(
        (status = 200, description = "The suggested configuration. JSON by default, plain text for the other formats.", body = ResponseHttp),
        (status = 500, description = "An invalid argument. v1 reports every input error as 500."),
    )
)]
async fn get_config(caller: Caller) -> Response {
    let call = match Call::parse(caller.query()) {
        Ok(call) => call,
        Err(error) => return invalid(&caller, "could not parse args", &error),
    };
    let categories = match call.categories(call.pgbadger.as_deref()) {
        Ok(categories) => categories,
        Err(error) => return invalid(&caller, "could not process config", &error),
    };
    if call.format == "json" {
        return success(&caller, categories);
    }

    let self_link = format!("{}\n", caller.self_link());
    let body = v1::export(
        &call.format,
        &categories,
        call.input.pg_version,
        &build::pretty(),
        &[self_link],
    );
    ([(CONTENT_TYPE, "text/plain; charset=utf-8")], body).into_response()
}

#[derive(Serialize)]
struct EnvironmentConfig {
    environment: Profile,
    configuration: Vec<v1::Category>,
}

/// Get Configuration for every profile
///
/// Computes the input and suggests a tuning configuration for each of the
/// supported profiles.
#[utoipa::path(
    get,
    path = "/v1/tuning/get-config-all-environments",
    tag = "tuning",
    params(ConfigParams),
    responses(
        (status = 200, description = "The suggested configuration of each profile.", body = ResponseHttp),
        (status = 500, description = "An invalid argument. v1 reports every input error as 500."),
    )
)]
async fn get_config_all_environments(caller: Caller) -> Response {
    let mut call = match Call::parse(caller.query()) {
        Ok(call) => call,
        Err(error) => return invalid(&caller, "could not parse args", &error),
    };
    let mut data = Vec::with_capacity(Profile::ALL.len());
    for profile in Profile::ALL {
        call.input.profile = *profile;
        match call.categories(None) {
            Ok(configuration) => {
                data.push(EnvironmentConfig {
                    environment: *profile,
                    configuration,
                });
            }
            Err(error) => return invalid(&caller, "could not process config", &error),
        }
    }
    success(&caller, data)
}

/// Lists all environments
///
/// Lists the supported environment profiles.
#[utoipa::path(
    get,
    path = "/v1/tuning/list-environments",
    tag = "tuning",
    responses((status = 200, description = "The profile names.", body = ResponseHttp))
)]
async fn list_environments(caller: Caller) -> Response {
    success(&caller, Profile::ALL)
}

/// Get API version
///
/// Returns the current API version and build metadata.
#[utoipa::path(
    get,
    path = "/v1/version",
    tag = "version",
    responses((status = 200, description = "The version of this build.", body = ResponseHttp))
)]
async fn version(caller: Caller) -> Response {
    let data = json!({"version": build::TAG, "build": build::COMMIT, "pretty": build::pretty()});
    let mut response = success(&caller, data);
    response.headers_mut().insert(
        CACHE_CONTROL,
        "public, max-age=3600".parse().expect("a header value"),
    );
    response
}

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "PGConfig API",
        description = "PostgreSQL configuration tuning. REST v1 is stable and stays available for the installers that call it."
    ),
    paths(get_config, get_config_all_environments, list_environments, version),
    components(schemas(ResponseHttp))
)]
pub(crate) struct ApiDoc;
