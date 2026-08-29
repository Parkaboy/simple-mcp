// src/main.rs
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    tool, tool_handler, tool_router,
    ServerHandler, ServiceExt,
    transport::stdio,
};
use schemars::JsonSchema;
use serde::Deserialize;

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

    #[tool(description = "Current forecast for a city")]
    async fn get_forecast(&self, Parameters(args): Parameters<ForecastArgs>) -> String {
        // Call a real weather API here; a canned string keeps the example runnable.
        format!("Forecast for {}: 18C, clear", args.city)
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
