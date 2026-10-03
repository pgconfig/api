//! The MCP endpoint over stateless Streamable HTTP. The contract is
//! `docs/mcp.md`. `recommend_postgres_configuration` recommends settings;
//! `list_postgres_parameters` and `describe_postgres_parameter` read the
//! PostgreSQL manual's entries that ship with the binary.
//!
//! This module adapts the protocol to `pgconfig::tune` and to the parameter
//! documentation. It owns the tools' schemas, the mapping of problems to tool
//! errors, the origin policy, the timeout, and the call log. It owns no
//! tuning rule.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::Router;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    ACCESS_CONTROL_REQUEST_HEADERS, ACCESS_CONTROL_REQUEST_METHOD, ORIGIN, VARY,
};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use pgconfig::{
    ParameterDoc, PgMajor, PgVersion, RawTuningRequest, TuningRequest, TuningResult, build,
    parameter_doc, parameter_docs,
};
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation,
    JsonObject, ListToolsResult, PaginatedRequestParams, ServerCapabilities, ServerConfig, Tool,
    ToolAnnotations,
};
use rmcp::service::RequestContext;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use rmcp::{ErrorData, RoleServer, ServerHandler};
use serde_json::{Value, json};

const TOOL: &str = "recommend_postgres_configuration";

/// How every tool describes `postgres_version`.
const POSTGRES_VERSION_DESCRIPTION: &str = "PostgreSQL version as a string, such as 18.4, 17.10, or 9.6.24. Supported major versions: 9.1 to 9.6 and 10 to 18.";
const LIST_TOOL: &str = "list_postgres_parameters";
const DESCRIBE_TOOL: &str = "describe_postgres_parameter";

/// How long one tool call may run. The rules take microseconds, so this only
/// bounds a call that got stuck.
const TIMEOUT: Duration = Duration::from_secs(5);

/// The arguments the tool takes, in the order the documentation lists them.
const ARGUMENTS: [&str; 8] = [
    "total_ram",
    "total_cpu",
    "postgres_version",
    "profile",
    "disk_type",
    "os",
    "arch",
    "max_connections",
];

/// The `/mcp` route with its origin policy.
pub(crate) fn router(allowed_origins: Vec<String>) -> Router {
    let config = StreamableHttpServerConfig::default()
        // No session and no stream: every call is one POST and one JSON
        // response, so any replica can answer it.
        .with_legacy_session_mode(false)
        .with_json_response(true)
        .with_sse_keep_alive(None)
        // The Host check guards a local server against DNS rebinding. This
        // endpoint is public, anonymous, and read-only, and it has to answer
        // on whatever hostname it is deployed under.
        .disable_allowed_hosts()
        // The same policy as `official_origins_only`, enforced a second time
        // by the transport.
        .with_allowed_origins(allowed_origins.iter().map(|origin| with_port(origin)))
        .enforce_origin_validation();
    let service: StreamableHttpService<TuningServer, LocalSessionManager> =
        StreamableHttpService::new(|| Ok(TuningServer::default()), Default::default(), config);

    Router::new()
        .nest_service("/mcp", service)
        .layer(middleware::from_fn_with_state(
            Arc::new(allowed_origins),
            official_origins_only,
        ))
}

/// The origin with its port written out. A browser leaves the default port of
/// the scheme out of `Origin`, and the transport wants it stated: an entry
/// without a port matches any port there.
fn with_port(origin: &str) -> String {
    let default_port = match origin.split_once("://") {
        Some(("https", _)) => 443,
        Some(("http", _)) => 80,
        _ => return origin.to_string(),
    };
    let host = origin.rsplit_once("://").map_or(origin, |(_, host)| host);
    // An IPv6 host has colons of its own, so only what follows `]` counts.
    let after_address = host.rsplit_once(']').map_or(host, |(_, rest)| rest);
    if after_address.contains(':') {
        origin.to_string()
    } else {
        format!("{origin}:{default_port}")
    }
}

/// A request without an `Origin` comes from a native client and passes. A
/// browser sends one, and only the configured origins get through: an
/// unrelated website must not be able to use a visitor's browser to call the
/// endpoint.
async fn official_origins_only(
    State(allowed): State<Arc<Vec<String>>>,
    request: Request,
    next: Next,
) -> Response {
    let Some(origin) = request.headers().get(ORIGIN).cloned() else {
        return next.run(request).await;
    };
    let official = origin.to_str().is_ok_and(|origin| {
        allowed
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(origin))
    });
    if !official {
        return (
            StatusCode::FORBIDDEN,
            "Forbidden: this origin may not call the MCP endpoint",
        )
            .into_response();
    }

    let headers = request.headers();
    let preflight =
        request.method() == Method::OPTIONS && headers.contains_key(ACCESS_CONTROL_REQUEST_METHOD);
    let mut response = if preflight {
        let mut response = Response::new(Body::empty());
        *response.status_mut() = StatusCode::NO_CONTENT;
        let out = response.headers_mut();
        out.insert(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("POST"),
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
        .insert(ACCESS_CONTROL_ALLOW_ORIGIN, origin);
    response
        .headers_mut()
        .insert(VARY, HeaderValue::from_static("Origin"));
    response
}

#[derive(Clone)]
struct TuningServer {
    tune: fn(&TuningRequest) -> TuningResult,
    timeout: Duration,
}

impl Default for TuningServer {
    fn default() -> Self {
        Self {
            tune: pgconfig::tune,
            timeout: TIMEOUT,
        }
    }
}

/// How a call ended, with what the log needs.
enum Outcome {
    Recommended(Box<TuningResult>),
    /// The arguments cannot be used. The message tells the caller what to fix.
    Invalid {
        message: String,
        missing_fields: Vec<&'static str>,
    },
    TimedOut,
}

impl TuningServer {
    async fn recommend(&self, arguments: &JsonObject) -> Outcome {
        let request = match tuning_request(arguments) {
            Ok(request) => request,
            Err(outcome) => return outcome,
        };
        let tune = self.tune;
        let work = tokio::task::spawn_blocking(move || tune(&request));
        match tokio::time::timeout(self.timeout, work).await {
            Ok(Ok(result)) => Outcome::Recommended(Box::new(result)),
            Ok(Err(_)) | Err(_) => Outcome::TimedOut,
        }
    }

    /// Runs one call and writes its log line. The line never carries the
    /// arguments: they describe someone's server.
    async fn call(&self, arguments: &JsonObject) -> CallToolResult {
        let started = Instant::now();
        let outcome = self.recommend(arguments).await;
        let duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);

        match outcome {
            Outcome::Recommended(result) => {
                tracing::info!(
                    tool = TOOL,
                    status = "ok",
                    duration_ms,
                    assumptions = result.assumptions.len(),
                    warnings = result.warnings.len(),
                    server_version = build::TAG,
                    "tool call"
                );
                let mut structured = serde_json::to_value(&*result).expect("a result is JSON");
                structured["application_version"] = Value::from(build::pretty());
                CallToolResult::structured(structured)
            }
            Outcome::Invalid {
                message,
                missing_fields,
            } => {
                tracing::warn!(
                    tool = TOOL,
                    status = "error",
                    error_code = "invalid_request",
                    missing_fields = missing_fields.join(","),
                    duration_ms,
                    server_version = build::TAG,
                    "tool call"
                );
                CallToolResult::error(vec![ContentBlock::text(message)])
            }
            Outcome::TimedOut => {
                tracing::error!(
                    tool = TOOL,
                    status = "error",
                    error_code = "timeout",
                    duration_ms,
                    server_version = build::TAG,
                    "tool call"
                );
                CallToolResult::error(vec![ContentBlock::text(format!(
                    "The recommendation did not finish within {} seconds. Call the tool again with the same arguments.",
                    self.timeout.as_secs()
                ))])
            }
        }
    }
}

/// Builds the Tuning Request from the tool arguments. Every problem is
/// collected, so one error lets the caller fix the whole call.
fn tuning_request(arguments: &JsonObject) -> Result<TuningRequest, Outcome> {
    let mut problems = Vec::new();
    let mut mistyped = Vec::new();

    for name in arguments
        .keys()
        .filter(|name| !ARGUMENTS.contains(&name.as_str()))
    {
        problems.push(format!(
            "{name} is not an argument of this tool. The arguments are {}, and {}.",
            ARGUMENTS[..ARGUMENTS.len() - 1].join(", "),
            ARGUMENTS[ARGUMENTS.len() - 1]
        ));
    }

    let mut text = |name: &'static str, example: &str, why: &str| match arguments.get(name) {
        None | Some(Value::Null) => None,
        Some(Value::String(text)) => Some(text.clone()),
        Some(_) => {
            problems.push(format!(
                "{name} must be a string, such as \"{example}\"{why}."
            ));
            mistyped.push(name);
            None
        }
    };
    let total_ram = text("total_ram", "16GB", "");
    let postgres_version = text(
        "postgres_version",
        "18.4",
        ": a number would turn 17.10 into 17.1",
    );
    let profile = text("profile", "WEB", "");
    let disk_type = text("disk_type", "SSD", "");
    let os = text("os", "linux", "");
    let arch = text("arch", "amd64", "");

    let mut integer = |name: &'static str, example: i64| match arguments.get(name) {
        None | Some(Value::Null) => None,
        Some(value) => value.as_i64().or_else(|| {
            problems.push(format!("{name} must be an integer, such as {example}."));
            mistyped.push(name);
            None
        }),
    };
    let total_cpu = integer("total_cpu", 8);
    let max_connections = integer("max_connections", 100);

    let raw = RawTuningRequest {
        total_ram,
        total_cpu,
        postgres_version,
        profile,
        disk_type,
        os,
        arch,
        max_connections,
    };
    let mut missing_fields = Vec::new();
    let request = match TuningRequest::try_from(raw) {
        Ok(request) => Some(request),
        Err(error) => {
            // A mistyped argument was read as absent. Its type problem is
            // already listed, so the "is required" that follows is dropped.
            for problem in error.problems {
                if !mistyped.contains(&problem.field) {
                    if problem.missing {
                        missing_fields.push(problem.field);
                    }
                    problems.push(problem.message);
                }
            }
            None
        }
    };

    match request {
        Some(request) if problems.is_empty() => Ok(request),
        _ => {
            // Arguments read in the order of ARGUMENTS, so the message does too.
            problems.sort_by_key(|problem| {
                ARGUMENTS
                    .iter()
                    .position(|name| problem.starts_with(name))
                    .unwrap_or(usize::MAX)
            });
            Err(Outcome::Invalid {
                message: format!("Invalid tuning request. {}", problems.join(" ")),
                missing_fields,
            })
        }
    }
}

impl ServerHandler for TuningServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("pgconfig", build::TAG))
            .with_instructions(
                "Recommends PostgreSQL configuration values for a server, and documents PostgreSQL parameters. Call recommend_postgres_configuration with the server's memory, logical CPU count, and PostgreSQL version. Call describe_postgres_parameter for what a parameter does in a version, and list_postgres_parameters to find a name.",
            )
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult::with_all_items(tools()))
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        tools().into_iter().find(|tool| tool.name == name)
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let arguments = request.arguments.unwrap_or_default();
        match request.name.as_ref() {
            TOOL => Ok(self.call(&arguments).await.into()),
            LIST_TOOL => {
                Ok(run_documentation_tool(LIST_TOOL, || list_parameters(&arguments)).into())
            }
            DESCRIBE_TOOL => {
                Ok(run_documentation_tool(DESCRIBE_TOOL, || describe_parameter(&arguments)).into())
            }
            name => Err(ErrorData::invalid_params(
                format!("Unknown tool: {name}"),
                None,
            )),
        }
    }
}

/// Every tool, in the order discovery lists them.
fn tools() -> Vec<Tool> {
    vec![tool(), list_tool(), describe_tool()]
}

/// Runs a call to a documentation tool and writes its log line. These tools
/// read data compiled into the binary, so they need no timeout.
fn run_documentation_tool(
    tool: &'static str,
    run: impl FnOnce() -> Result<Value, String>,
) -> CallToolResult {
    let started = Instant::now();
    let outcome = run();
    let duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    match outcome {
        Ok(structured) => {
            tracing::info!(
                tool,
                status = "ok",
                duration_ms,
                server_version = build::TAG,
                "tool call"
            );
            CallToolResult::structured(structured)
        }
        Err(message) => {
            tracing::warn!(
                tool,
                status = "error",
                error_code = "invalid_request",
                duration_ms,
                server_version = build::TAG,
                "tool call"
            );
            CallToolResult::error(vec![ContentBlock::text(message)])
        }
    }
}

/// One text argument of a documentation tool.
struct Argument {
    name: &'static str,
    required: bool,
    /// A valid value, for the error messages.
    example: &'static str,
    /// What the value is, for the error messages.
    what: &'static str,
}

/// How every documentation tool takes the PostgreSQL version.
const POSTGRES_VERSION: Argument = Argument {
    name: "postgres_version",
    required: true,
    example: "18.4",
    what: "the PostgreSQL version",
};

/// The text arguments of a documentation tool. Every problem is collected,
/// in argument order, so one error lets the caller fix the whole call.
/// Every documentation tool takes `postgres_version`, which is parsed where
/// it stands so that its problem keeps its place.
struct Arguments {
    values: HashMap<&'static str, String>,
    major: Option<PgMajor>,
    problems: Vec<String>,
}

impl Arguments {
    fn read(arguments: &JsonObject, known: &[Argument]) -> Self {
        let mut read = Arguments {
            values: HashMap::new(),
            major: None,
            problems: Vec::new(),
        };
        for argument in known {
            let name = argument.name;
            match arguments.get(name) {
                None | Some(Value::Null) if argument.required => read.problems.push(format!(
                    "{name} is required: {}, such as {}.",
                    argument.what, argument.example
                )),
                None | Some(Value::Null) => {}
                Some(Value::String(text)) => {
                    if name == POSTGRES_VERSION.name {
                        match PgVersion::parse(text) {
                            Ok(version) => read.major = Some(version.major()),
                            Err(error) => read.problems.push(format!("{name}: {error}")),
                        }
                    }
                    read.values.insert(name, text.clone());
                }
                Some(_) => read.problems.push(format!(
                    "{name} must be a string, such as \"{}\".",
                    argument.example
                )),
            }
        }

        let names: Vec<&str> = known.iter().map(|argument| argument.name).collect();
        let listed = match names.as_slice() {
            [only] => only.to_string(),
            [first, second] => format!("{first} and {second}"),
            [rest @ .., last] => format!("{}, and {last}", rest.join(", ")),
            [] => String::new(),
        };
        for name in arguments
            .keys()
            .filter(|name| !names.contains(&name.as_str()))
        {
            read.problems.push(format!(
                "{name} is not an argument of this tool. The arguments are {listed}."
            ));
        }
        read
    }

    /// The values by name and the major version, or the error that lists
    /// every problem.
    fn finish(self) -> Result<(HashMap<&'static str, String>, PgMajor), String> {
        match self.major {
            Some(major) if self.problems.is_empty() => Ok((self.values, major)),
            _ => Err(format!("Invalid request. {}", self.problems.join(" "))),
        }
    }
}

fn list_parameters(arguments: &JsonObject) -> Result<Value, String> {
    let (values, major) = Arguments::read(
        arguments,
        &[
            POSTGRES_VERSION,
            Argument {
                name: "category",
                required: false,
                example: "memory",
                what: "a part of a category",
            },
            Argument {
                name: "search",
                required: false,
                example: "vacuum",
                what: "a part of a name or description",
            },
        ],
    )
    .finish()?;
    let contains = |text: Option<&str>, part: Option<&String>| match part {
        None => true,
        Some(part) => text.is_some_and(|text| text.to_lowercase().contains(&part.to_lowercase())),
    };
    let (category, search) = (values.get("category"), values.get("search"));
    let parameters: Vec<Value> = parameter_docs(major)
        .iter()
        .filter(|doc| contains(doc.category, category))
        .filter(|doc| contains(Some(doc.name), search) || contains(doc.short_desc, search))
        .map(|doc| {
            let mut entry = serde_json::Map::new();
            entry.insert("name".into(), Value::from(doc.name));
            optional(&mut entry, "category", doc.category);
            optional(&mut entry, "short_desc", doc.short_desc);
            Value::Object(entry)
        })
        .collect();
    Ok(json!({
        "postgres_version": values[POSTGRES_VERSION.name],
        "parameters": parameters,
    }))
}

fn describe_parameter(arguments: &JsonObject) -> Result<Value, String> {
    let (values, major) = Arguments::read(
        arguments,
        &[
            Argument {
                name: "name",
                required: true,
                example: "work_mem",
                what: "the parameter name",
            },
            POSTGRES_VERSION,
        ],
    )
    .finish()?;
    let name = &values["name"];
    let doc = parameter_doc(major, name).ok_or_else(|| unknown_parameter(name, major))?;

    let mut result = serde_json::Map::new();
    result.insert(
        "postgres_version".into(),
        Value::from(values[POSTGRES_VERSION.name].clone()),
    );
    result.insert("name".into(), Value::from(doc.name));
    describe(&mut result, doc);
    Ok(Value::Object(result))
}

fn describe(result: &mut JsonObject, doc: &ParameterDoc) {
    optional(result, "type", doc.param_type);
    optional(result, "category", doc.category);
    optional(result, "short_desc", doc.short_desc);
    optional(result, "extra_desc", doc.extra_desc);
    optional(result, "context", doc.context);
    optional(result, "unit", doc.unit);
    optional(result, "default", doc.default);
    optional(result, "min", doc.min);
    optional(result, "max", doc.max);
    if !doc.values.is_empty() {
        result.insert("values".into(), Value::from(doc.values.to_vec()));
    }
    result.insert("url".into(), Value::from(doc.url));
    result.insert("documentation".into(), Value::from(doc.text()));
}

fn optional(object: &mut JsonObject, key: &str, value: Option<&str>) {
    if let Some(value) = value {
        object.insert(key.into(), Value::from(value));
    }
}

/// Why `name` has no entry: the major versions that have it, or the names
/// it looks like.
fn unknown_parameter(name: &str, major: PgMajor) -> String {
    let mut message = format!("{name} is not a parameter of PostgreSQL {major}.");
    let documented: Vec<PgMajor> = PgMajor::supported()
        .filter(|other| parameter_doc(*other, name).is_some())
        .collect();
    if let (Some(first), Some(last)) = (documented.first(), documented.last()) {
        let range = if first == last {
            format!("only in PostgreSQL {first}")
        } else {
            format!("in PostgreSQL {first} to {last}")
        };
        message.push_str(&format!(" The manual documents it {range}."));
    } else {
        let lower = name.to_lowercase();
        let mut close: Vec<(usize, &str)> = parameter_docs(major)
            .iter()
            .map(|doc| (distance(&lower, &doc.name.to_lowercase()), doc.name))
            .filter(|(distance, _)| *distance <= 2)
            .collect();
        close.sort();
        if !close.is_empty() {
            let names: Vec<&str> = close.iter().take(3).map(|(_, name)| *name).collect();
            message.push_str(&format!(" Did you mean {}?", names.join(", ")));
        }
    }
    message.push_str(" Call list_postgres_parameters to find a name.");
    message
}

/// The Levenshtein distance between two names.
fn distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut diagonal = row[0];
        row[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let above = row[j + 1];
            row[j + 1] = (above + 1)
                .min(row[j] + 1)
                .min(diagonal + usize::from(ca != *cb));
            diagonal = above;
        }
    }
    row[b.len()]
}

fn tool() -> Tool {
    let mut tool = Tool::new(
        TOOL,
        "Recommends PostgreSQL configuration values for one server. Give the memory dedicated to PostgreSQL, the logical CPU count, and the PostgreSQL version. The result lists each parameter with its value and the reason for it, the defaults assumed for omitted arguments, and warnings. It is deterministic and changes nothing.",
        object(input_schema()),
    );
    tool.output_schema = Some(Arc::new(object(output_schema())));
    read_only(tool, "Recommend PostgreSQL configuration")
}

/// The annotations every tool shares: it reads and changes nothing.
fn read_only(mut tool: Tool, title: &str) -> Tool {
    tool.title = Some(title.to_string());
    tool.annotations = Some(
        ToolAnnotations::new()
            .read_only(true)
            .destructive(false)
            .idempotent(true)
            .open_world(false),
    );
    tool
}

fn list_tool() -> Tool {
    let text = |description: &str| json!({"type": "string", "description": description});
    let mut tool = Tool::new(
        LIST_TOOL,
        "Lists the configuration parameters the PostgreSQL manual documents for one PostgreSQL version, with each one's category and short description. Narrow the list by category or by text. It reads the documentation that ships with pgconfig and changes nothing.",
        object(json!({
            "type": "object",
            "properties": {
                "postgres_version": text(POSTGRES_VERSION_DESCRIPTION),
                "category": text("Keeps the parameters whose category contains this text, in any case, such as memory or Write-Ahead Log."),
                "search": text("Keeps the parameters whose name or short description contains this text, in any case, such as vacuum."),
            },
            "required": ["postgres_version"],
            "additionalProperties": false,
        })),
    );
    tool.output_schema = Some(Arc::new(object(json!({
        "type": "object",
        "properties": {
            "postgres_version": text("As supplied."),
            "parameters": {
                "type": "array",
                "description": "Sorted by name. Empty when nothing matches.",
                "items": {
                    "type": "object",
                    "properties": {
                        "name": text("The parameter name."),
                        "category": text("The category pg_settings shows."),
                        "short_desc": text("One line on what the parameter does."),
                    },
                    "required": ["name"],
                },
            },
        },
        "required": ["postgres_version", "parameters"],
    }))));
    read_only(tool, "List PostgreSQL parameters")
}

fn describe_tool() -> Tool {
    let text = |description: &str| json!({"type": "string", "description": description});
    let mut tool = Tool::new(
        DESCRIBE_TOOL,
        "Returns the PostgreSQL manual's entry for one configuration parameter in one PostgreSQL version: its type, its context, which tells whether a change needs a restart, its unit, default, limits, and accepted values, and the full text in Markdown, with a link to the official page. It reads the documentation that ships with pgconfig and changes nothing.",
        object(json!({
            "type": "object",
            "properties": {
                "name": text("The parameter name, in any case, such as work_mem."),
                "postgres_version": text(POSTGRES_VERSION_DESCRIPTION),
            },
            "required": ["name", "postgres_version"],
            "additionalProperties": false,
        })),
    );
    tool.output_schema = Some(Arc::new(object(json!({
        "type": "object",
        "properties": {
            "postgres_version": text("As supplied."),
            "name": text("The name as the manual writes it."),
            "type": text("boolean, integer, floating point, string, or enum."),
            "category": text("The category pg_settings shows."),
            "short_desc": text("One line on what the parameter does."),
            "extra_desc": text("More detail, when PostgreSQL has it."),
            "context": text("When a change takes effect: internal, postmaster (restart), sighup (reload), superuser-backend, backend, superuser, or user."),
            "unit": text("The unit of the default and the limits, such as kB, 8kB, or ms."),
            "default": text("The value PostgreSQL starts with, in the unit."),
            "min": text("The lowest accepted value, in the unit."),
            "max": text("The highest accepted value, in the unit."),
            "values": {"type": "array", "items": {"type": "string"}, "description": "The values an enum accepts."},
            "url": text("The entry in the PostgreSQL manual."),
            "documentation": text("The manual's text, in Markdown."),
        },
        "required": ["postgres_version", "name", "url", "documentation"],
    }))));
    read_only(tool, "Describe a PostgreSQL parameter")
}

fn object(schema: Value) -> JsonObject {
    match schema {
        Value::Object(object) => object,
        _ => unreachable!("a schema is a JSON object"),
    }
}

/// The values are described, not enumerated: a client that validated an
/// `enum` would reject the lowercase spellings the tool accepts.
fn input_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "total_ram": {
                "type": "string",
                "description": "Memory dedicated to PostgreSQL: a positive integer followed by B, KB, MB, GB, or TB, in any case. Examples: 16GB, 1536MB. Decimals are not accepted, so write 1536MB instead of 1.5GB.",
            },
            "total_cpu": {
                "type": "integer",
                "minimum": 1,
                "description": "Logical CPUs, hyperthreads included, as nproc reports them.",
            },
            "postgres_version": {
                "type": "string",
                "description": POSTGRES_VERSION_DESCRIPTION,
            },
            "profile": {
                "type": "string",
                "description": "Workload: WEB, OLTP, DW, MIXED, or DESKTOP, in any case. Defaults to WEB.",
            },
            "disk_type": {
                "type": "string",
                "description": "Storage: SSD, HDD, or SAN, in any case. Defaults to SSD.",
            },
            "os": {
                "type": "string",
                "description": "Operating system: linux, windows, unix, or darwin, in any case. Defaults to linux.",
            },
            "arch": {
                "type": "string",
                "description": "CPU architecture: 386, i686, amd64, x86-64, arm, or arm64, in any case. Defaults to amd64.",
            },
            "max_connections": {
                "type": "integer",
                "minimum": 1,
                "description": "Expected maximum number of connections. Defaults to 100.",
            },
        },
        "required": ["total_ram", "total_cpu", "postgres_version"],
        "additionalProperties": false,
    })
}

fn output_schema() -> Value {
    let text = |description: &str| json!({"type": "string", "description": description});
    json!({
        "type": "object",
        "properties": {
            "request": {
                "type": "object",
                "description": "The request the recommendations were computed from: every supplied argument normalized, and every omitted one filled in.",
                "properties": {
                    "os": text("linux, windows, unix, or darwin."),
                    "arch": text("386, amd64, arm, or arm64."),
                    "total_ram": text("Such as 16GB."),
                    "profile": text("WEB, OLTP, DW, MIXED, or DESKTOP."),
                    "disk_type": text("SSD, HDD, or SAN."),
                    "max_connections": {"type": "integer"},
                    "total_cpu": {"type": "integer"},
                    "postgres_version": text("As supplied."),
                },
                "required": ARGUMENTS,
            },
            "assumptions": {
                "type": "array",
                "description": "One entry per omitted argument that received a default.",
                "items": {
                    "type": "object",
                    "properties": {
                        "field": text("The omitted argument."),
                        "value": text("The value assumed for it."),
                        "message": text("The assumption in English."),
                    },
                    "required": ["field", "value", "message"],
                },
            },
            "warnings": {
                "type": "array",
                "description": "Concerns about a usable request. A warning does not invalidate the result.",
                "items": {
                    "type": "object",
                    "properties": {
                        "code": text("A stable identifier of the kind of concern."),
                        "message": text("The concern in English."),
                    },
                    "required": ["code", "message"],
                },
            },
            "recommendations": {
                "type": "object",
                "description": "Keyed by PostgreSQL parameter name. A parameter the release does not have is absent.",
                "additionalProperties": {
                    "type": "object",
                    "properties": {
                        "value": text("The value as written in postgresql.conf."),
                        "reason": text("How the value was reached, including any limit that changed it."),
                    },
                    "required": ["value", "reason"],
                },
            },
            "application_version": text("The pgconfig release that produced the result."),
        },
        "required": ["request", "assumptions", "warnings", "recommendations", "application_version"],
    })
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::Mutex;

    use super::*;

    fn arguments(value: Value) -> JsonObject {
        object(value)
    }

    fn complete() -> JsonObject {
        arguments(json!({
            "total_ram": "16GB",
            "total_cpu": 8,
            "postgres_version": "18.4",
            "profile": "OLTP",
            "disk_type": "SSD",
            "os": "linux",
            "arch": "amd64",
            "max_connections": 250,
        }))
    }

    fn text(result: &CallToolResult) -> &str {
        &result.content[0].as_text().expect("text content").text
    }

    /// Collects the log lines written while a test runs.
    #[derive(Clone, Default)]
    struct Log(Arc<Mutex<Vec<u8>>>);

    impl io::Write for Log {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Log {
        /// Captures the log of the current thread until the guard is dropped.
        fn capture() -> (Self, tracing::subscriber::DefaultGuard) {
            let log = Log::default();
            let writer = log.clone();
            let subscriber = tracing_subscriber::fmt()
                .with_ansi(false)
                .without_time()
                .with_writer(move || writer.clone())
                .finish();
            (log, tracing::subscriber::set_default(subscriber))
        }

        fn text(&self) -> String {
            String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
        }
    }

    #[tokio::test]
    async fn a_call_that_outlives_the_timeout_is_an_actionable_tool_error() {
        fn stuck(request: &TuningRequest) -> TuningResult {
            std::thread::sleep(Duration::from_millis(300));
            pgconfig::tune(request)
        }
        let server = TuningServer {
            tune: stuck,
            timeout: Duration::from_millis(20),
        };
        let started = Instant::now();

        let result = server.call(&complete()).await;

        assert!(
            started.elapsed() < Duration::from_millis(250),
            "the call waited for the work"
        );
        assert_eq!(result.is_error, Some(true));
        assert_eq!(
            text(&result),
            "The recommendation did not finish within 0 seconds. Call the tool again with the same arguments."
        );
    }

    #[test]
    fn an_origin_without_a_port_gets_the_default_of_its_scheme() {
        assert_eq!(
            with_port("https://pgconfig.org"),
            "https://pgconfig.org:443"
        );
        assert_eq!(with_port("http://localhost"), "http://localhost:80");
        assert_eq!(with_port("http://localhost:5173"), "http://localhost:5173");
        assert_eq!(with_port("https://[::1]"), "https://[::1]:443");
        assert_eq!(with_port("https://[::1]:8443"), "https://[::1]:8443");
        assert_eq!(with_port("null"), "null");
    }

    #[tokio::test]
    async fn the_default_timeout_is_five_seconds() {
        assert_eq!(TuningServer::default().timeout, Duration::from_secs(5));
    }

    #[tokio::test]
    async fn a_successful_call_logs_counts_and_no_arguments() {
        let (log, _guard) = Log::capture();

        let mut request = complete();
        request.remove("os");
        request.insert("max_connections".into(), json!(5000));
        TuningServer::default().call(&request).await;

        let line = log.text();
        for field in [
            "tool=\"recommend_postgres_configuration\"",
            "status=\"ok\"",
            "duration_ms=",
            "assumptions=1",
            "warnings=1",
            &format!("server_version=\"{}\"", build::TAG),
        ] {
            assert!(line.contains(field), "{field} is missing from: {line}");
        }
        for argument in ["16GB", "18.4", "OLTP", "5000", "amd64"] {
            assert!(
                !line.contains(argument),
                "the argument {argument} leaked into: {line}"
            );
        }
    }

    #[tokio::test]
    async fn a_failed_call_logs_a_stable_code_and_the_missing_fields() {
        let (log, _guard) = Log::capture();

        TuningServer::default()
            .call(&arguments(
                json!({"total_ram": "sixteen gigabytes", "profile": "bogus"}),
            ))
            .await;

        let line = log.text();
        for field in [
            "status=\"error\"",
            "error_code=\"invalid_request\"",
            "missing_fields=\"total_cpu,postgres_version\"",
            "duration_ms=",
        ] {
            assert!(line.contains(field), "{field} is missing from: {line}");
        }
        for argument in ["sixteen", "bogus"] {
            assert!(
                !line.contains(argument),
                "the argument {argument} leaked into: {line}"
            );
        }
    }

    #[tokio::test]
    async fn a_timeout_logs_its_own_code() {
        fn stuck(request: &TuningRequest) -> TuningResult {
            std::thread::sleep(Duration::from_millis(100));
            pgconfig::tune(request)
        }
        let (log, _guard) = Log::capture();

        TuningServer {
            tune: stuck,
            timeout: Duration::from_millis(5),
        }
        .call(&complete())
        .await;

        let line = log.text();
        assert!(line.contains("error_code=\"timeout\""), "{line}");
        assert!(!line.contains("16GB"), "{line}");
    }
}
