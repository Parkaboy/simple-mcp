// src/main.rs
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    tool, tool_handler, tool_router,
    ServerHandler, ServiceExt,
    transport::stdio,
};
use schemars::JsonSchema;
use serde::Deserialize;
use rmcp::model::{CallToolResult, ContentBlock};
    use rmcp::ErrorData as McpError;

#[derive(Debug, Deserialize, JsonSchema)]
struct ForecastArgs {
    /// The city to get the forecast for.
    city: String,
}

#[derive(Clone)]
struct WeatherServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl WeatherServer {
    fn new() -> Self {
        Self { tool_router: Self::tool_router() }
    }



#[tool(description = "Fetch the forecast, or report why it failed")]
async fn get_forecast(&self, Parameters(args): Parameters<ForecastArgs>) -> CallToolResult {
    

    if args.city.is_empty() {
        return Err(McpError::invalid_params("city must not be empty", None));
    }
    
    match fetch_upstream(&args.city).await {
        Ok(text) => CallToolResult::success(vec![ContentBlock::text(text)]),
        Err(e) => CallToolResult::error(vec![ContentBlock::text(
            format!("weather lookup failed: {e}"),
        )]),
    }
}

}

#[tool_handler]
impl ServerHandler for WeatherServer {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = WeatherServer::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
