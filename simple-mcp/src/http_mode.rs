//! Optional Streamable HTTP transport setup for demonstrations.
//!
//! The default transport remains stdio; HTTP is selected with `--http`.

use std::sync::Arc;

use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use tokio_util::sync::CancellationToken;

/// Runs the server over Streamable HTTP on localhost.
pub(crate) async fn run_http() -> anyhow::Result<()> {
    let cancellation_token = CancellationToken::new();
    let service = StreamableHttpService::new(
        || Ok(crate::WeatherServer::new()),
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default()
            .with_legacy_session_mode(false)
            .with_json_response(true)
            .with_cancellation_token(cancellation_token.clone()),
    );
    let app = axum::Router::new().nest_service("/mcp", service);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;
    eprintln!("MCP HTTP server listening on http://127.0.0.1:8000/mcp");
    axum::serve(listener, app)
        .with_graceful_shutdown(async move { cancellation_token.cancelled_owned().await })
        .await?;
    Ok(())
}
