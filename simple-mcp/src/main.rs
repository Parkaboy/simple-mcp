// src/main.rs
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, ContentBlock},
    tool, tool_handler, tool_router,
    ServerHandler, ServiceExt,
    transport::stdio,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use rmcp::ErrorData as McpError;

#[derive(Debug, Deserialize, JsonSchema)]
struct ForecastArgs {
    /// The city to get the forecast for.
    city: String,
}

#[derive(Debug, Serialize, JsonSchema)]
struct Forecast {
    city: String,
    temp_c: f64,
    summary: String,
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

    #[tool(description = "Current forecast for a city")]
    async fn get_forecast(&self, Parameters(args): Parameters<ForecastArgs>) -> CallToolResult {
        let city = args.city.trim();
        if city.is_empty() {
            return CallToolResult::error(vec![ContentBlock::text(
                "city must not be empty".to_string(),
            )]);
        }

        match fetch_upstream(city).await {
            Ok(forecast) => match serde_json::to_value(forecast) {
                Ok(data) => CallToolResult::structured(data),
                Err(error) => CallToolResult::error(vec![ContentBlock::text(format!(
                    "could not serialize forecast: {error}"
                ))]),
            },
            Err(error) => CallToolResult::error(vec![ContentBlock::text(format!(
                "weather lookup failed: {error}"
            ))]),
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

async fn fetch_upstream(city: &str) -> Result<Forecast, McpError> {
    // A canned response keeps this test server independent of external services.
    Ok(Forecast {
        city: city.to_string(),
        temp_c: 18.0,
        summary: "clear".to_string(),
    })
}
