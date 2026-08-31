// src/main.rs
mod weather;

use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, ContentBlock},
    tool, tool_handler, tool_router,
    transport::stdio,
};
use schemars::JsonSchema;
use serde::Deserialize;

use weather::fetch_upstream;

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
        Self {
            tool_router: Self::tool_router(),
        }
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
