//! The MCP endpoint, exercised over HTTP with the official client and with
//! raw requests where the client would get in the way.

use std::net::SocketAddr;

use pgconfig_server::{Config, app_with};
use rmcp::model::{CallToolRequestParams, CallToolResult};
use rmcp::service::RunningService;
use rmcp::transport::StreamableHttpClientTransport;
use rmcp::{RoleClient, ServiceExt};
use serde_json::{Value, json};
use tokio::net::TcpListener;

const TOOL: &str = "recommend_postgres_configuration";

/// Serves the application on a free port for the length of the test.
async fn serve(config: Config) -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app_with(config)).await });
    address
}

async fn connect(address: SocketAddr) -> RunningService<RoleClient, ()> {
    let transport = StreamableHttpClientTransport::from_uri(format!("http://{address}/mcp"));
    ().serve(transport).await.expect("the MCP handshake")
}

async fn call(client: &RunningService<RoleClient, ()>, arguments: Value) -> CallToolResult {
    let arguments = arguments.as_object().cloned().unwrap();
    client
        .call_tool(CallToolRequestParams::new(TOOL).with_arguments(arguments))
        .await
        .expect("a tool result, not a protocol error")
}

/// The text of a result's only content block.
fn text(result: &CallToolResult) -> String {
    assert_eq!(result.content.len(), 1, "{result:?}");
    result.content[0]
        .as_text()
        .expect("text content")
        .text
        .clone()
}

fn complete() -> Value {
    json!({
        "total_ram": "16GB",
        "total_cpu": 8,
        "postgres_version": "18.4",
        "profile": "web",
        "disk_type": "ssd",
        "os": "Linux",
        "arch": "x86-64",
        "max_connections": 100,
    })
}

#[tokio::test]
async fn the_server_identifies_as_pgconfig_with_the_release_version() {
    let client = connect(serve(Config::default()).await).await;

    let server = client.peer_info().expect("the server info");

    let identity = server.server_info.as_ref().expect("the server identity");
    assert_eq!(identity.name, "pgconfig");
    assert_eq!(identity.version, pgconfig::build::TAG);
    assert!(server.capabilities.tools.is_some());
}

#[tokio::test]
async fn discovery_lists_one_read_only_tool() {
    let client = connect(serve(Config::default()).await).await;

    let tools = client.list_all_tools().await.unwrap();

    assert_eq!(tools.len(), 1);
    let tool = &tools[0];
    assert_eq!(tool.name, TOOL);
    let annotations = tool.annotations.as_ref().expect("annotations");
    assert_eq!(annotations.read_only_hint, Some(true));
    assert_eq!(annotations.idempotent_hint, Some(true));
    assert_eq!(annotations.destructive_hint, Some(false));
    assert_eq!(annotations.open_world_hint, Some(false));
    assert_eq!(
        tool.input_schema["required"],
        json!(["total_ram", "total_cpu", "postgres_version"])
    );
    let output = tool.output_schema.as_ref().expect("an output schema");
    assert_eq!(
        output["required"],
        json!([
            "request",
            "assumptions",
            "warnings",
            "recommendations",
            "application_version"
        ])
    );
}

#[tokio::test]
async fn a_complete_request_returns_the_structured_result() {
    let client = connect(serve(Config::default()).await).await;

    let result = call(&client, complete()).await;

    assert_eq!(result.is_error, Some(false));
    let structured = result
        .structured_content
        .clone()
        .expect("structured content");
    assert_eq!(
        structured["request"],
        json!({
            "os": "linux",
            "arch": "amd64",
            "total_ram": "16GB",
            "profile": "WEB",
            "disk_type": "SSD",
            "max_connections": 100,
            "total_cpu": 8,
            "postgres_version": "18.4",
        })
    );
    assert_eq!(structured["assumptions"], json!([]));
    assert_eq!(structured["warnings"], json!([]));
    assert_eq!(
        structured["recommendations"]["shared_buffers"],
        json!({"value": "4GB", "reason": "Set to 4GB from the memory share for the WEB profile."})
    );
    assert_eq!(structured["application_version"], pgconfig::build::pretty());
    assert!(
        structured["recommendations"]
            .get("listen_addresses")
            .is_none()
    );
}

#[tokio::test]
async fn the_text_content_mirrors_the_structured_result() {
    let client = connect(serve(Config::default()).await).await;

    let result = call(&client, complete()).await;

    let mirrored: Value = serde_json::from_str(&text(&result)).expect("JSON text");
    assert_eq!(Some(mirrored), result.structured_content);
}

#[tokio::test]
async fn omitted_optional_facts_come_back_as_assumptions() {
    let client = connect(serve(Config::default()).await).await;

    let result = call(
        &client,
        json!({"total_ram": "8GB", "total_cpu": 4, "postgres_version": "17.10"}),
    )
    .await;

    let structured = result.structured_content.expect("structured content");
    let assumed: Vec<&str> = structured["assumptions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|assumption| assumption["field"].as_str().unwrap())
        .collect();
    assert_eq!(
        assumed,
        ["profile", "disk_type", "os", "arch", "max_connections"]
    );
    assert_eq!(structured["request"]["postgres_version"], "17.10");
}

#[tokio::test]
async fn missing_and_invalid_facts_are_one_actionable_tool_error() {
    let client = connect(serve(Config::default()).await).await;

    let result = call(&client, json!({"total_ram": "16"})).await;

    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(
        text(&result),
        "Invalid tuning request. total_ram: \"16\" has no unit. Use a positive integer followed by B, KB, MB, GB, or TB, such as 16GB or 1536MB. total_cpu is required: the number of logical CPUs, such as 8. postgres_version is required: the PostgreSQL version, such as 18.4."
    );
}

#[tokio::test]
async fn a_wrongly_typed_argument_is_a_tool_error_that_says_what_to_send() {
    let client = connect(serve(Config::default()).await).await;

    let result = call(
        &client,
        json!({"total_ram": 16, "total_cpu": "8", "postgres_version": 17.10, "max_connections": 1.5}),
    )
    .await;

    assert_eq!(result.is_error, Some(true));
    assert_eq!(
        text(&result),
        "Invalid tuning request. total_ram must be a string, such as \"16GB\". total_cpu must be an integer, such as 8. postgres_version must be a string, such as \"18.4\": a number would turn 17.10 into 17.1. max_connections must be an integer, such as 100."
    );
}

#[tokio::test]
async fn an_unknown_argument_is_a_tool_error_that_lists_the_known_ones() {
    let client = connect(serve(Config::default()).await).await;

    let mut arguments = complete();
    arguments["ram"] = json!("16GB");
    let result = call(&client, arguments).await;

    assert_eq!(result.is_error, Some(true));
    assert_eq!(
        text(&result),
        "Invalid tuning request. ram is not an argument of this tool. The arguments are total_ram, total_cpu, postgres_version, profile, disk_type, os, arch, and max_connections."
    );
}

#[tokio::test]
async fn an_invalid_optional_fact_is_an_error_not_a_default() {
    let client = connect(serve(Config::default()).await).await;

    let mut arguments = complete();
    arguments["disk_type"] = json!("nvme");
    let result = call(&client, arguments).await;

    assert_eq!(result.is_error, Some(true));
    assert_eq!(
        text(&result),
        "Invalid tuning request. disk_type: \"nvme\" is not a known disk type. Use one of: SSD, HDD, SAN."
    );
}

#[tokio::test]
async fn an_unknown_tool_is_a_protocol_error() {
    let client = connect(serve(Config::default()).await).await;

    let error = client
        .call_tool(CallToolRequestParams::new("drop_database"))
        .await
        .expect_err("a protocol error");

    assert!(error.to_string().contains("drop_database"), "{error}");
}

/// Sends one JSON-RPC message without the client library.
async fn post(address: SocketAddr, origin: Option<&str>, body: Value) -> reqwest::Response {
    let mut request = reqwest::Client::new()
        .post(format!("http://{address}/mcp"))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream");
    if let Some(origin) = origin {
        request = request.header("Origin", origin);
    }
    request.json(&body).send().await.unwrap()
}

fn initialize() -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"},
        },
    })
}

fn tools_call(arguments: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {"name": TOOL, "arguments": arguments},
    })
}

#[tokio::test]
async fn a_native_client_without_an_origin_is_accepted() {
    let address = serve(Config::default()).await;

    let response = post(address, None, initialize()).await;

    assert_eq!(response.status(), 200);
    assert!(
        response
            .headers()
            .get("access-control-allow-origin")
            .is_none()
    );
}

#[tokio::test]
async fn a_browser_on_an_official_origin_is_accepted() {
    let address = serve(Config::default()).await;

    let response = post(address, Some("https://pgconfig.org"), initialize()).await;

    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers()["access-control-allow-origin"],
        "https://pgconfig.org"
    );
}

#[tokio::test]
async fn a_browser_on_any_other_origin_is_rejected() {
    let address = serve(Config::default()).await;

    let response = post(
        address,
        Some("https://evil.example"),
        tools_call(complete()),
    )
    .await;

    assert_eq!(response.status(), 403);
    assert!(
        response
            .headers()
            .get("access-control-allow-origin")
            .is_none()
    );
    let body = response.text().await.unwrap();
    assert!(!body.contains("shared_buffers"), "{body}");
}

#[tokio::test]
async fn a_preflight_from_another_origin_is_rejected_too() {
    let address = serve(Config::default()).await;

    let response = reqwest::Client::new()
        .request(reqwest::Method::OPTIONS, format!("http://{address}/mcp"))
        .header("Origin", "https://evil.example")
        .header("Access-Control-Request-Method", "POST")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 403);
}

#[tokio::test]
async fn the_allowed_origins_are_configurable() {
    let config = Config {
        mcp_allowed_origins: vec!["https://staging.example".into()],
    };
    let address = serve(config).await;

    let allowed = post(address, Some("https://staging.example"), initialize()).await;
    let official = post(address, Some("https://pgconfig.org"), initialize()).await;

    assert_eq!(allowed.status(), 200);
    assert_eq!(official.status(), 403);
}

#[tokio::test]
async fn a_call_needs_no_session() {
    let address = serve(Config::default()).await;

    let handshake = post(address, None, initialize()).await;
    let call = post(address, None, tools_call(complete())).await;

    assert!(handshake.headers().get("mcp-session-id").is_none());
    assert_eq!(call.status(), 200);
    assert!(
        call.headers()["content-type"]
            .to_str()
            .unwrap()
            .starts_with("application/json")
    );
    let body: Value = call.json().await.unwrap();
    assert_eq!(
        body["result"]["structuredContent"]["request"]["total_ram"],
        "16GB"
    );
    assert_eq!(body["result"]["isError"], false);
}

#[tokio::test]
async fn rest_v1_keeps_answering_any_origin() {
    let address = serve(Config::default()).await;

    let response = reqwest::Client::new()
        .get(format!("http://{address}/v1/tuning/list-environments"))
        .header("Origin", "https://evil.example")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(response.headers()["access-control-allow-origin"], "*");
}
