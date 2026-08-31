//! MCP prompts for reusable travel-weather instructions.

use rmcp::model::{
    GetPromptRequestParams, GetPromptResult, ListPromptsResult, PaginatedRequestParams, Prompt,
    PromptMessage, Role,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer};

/// Lists the travel-weather prompt available to MCP clients.
pub(crate) async fn list_prompts(
    _server: &crate::WeatherServer,
    _request: Option<PaginatedRequestParams>,
    _context: RequestContext<RoleServer>,
) -> Result<ListPromptsResult, ErrorData> {
    Ok(ListPromptsResult::with_all_items(vec![Prompt::new(
        "travel_weather_report",
        Some("Create a concise travel weather report"),
        None,
    )]))
}

/// Expands the travel-weather prompt with the requested city.
pub(crate) async fn get_prompt(
    _server: &crate::WeatherServer,
    request: GetPromptRequestParams,
    _context: RequestContext<RoleServer>,
) -> Result<rmcp::model::GetPromptResponse, ErrorData> {
    if request.name != "travel_weather_report" {
        return Err(ErrorData::invalid_params("unknown prompt", None));
    }
    let city = request
        .arguments
        .as_ref()
        .and_then(|arguments| arguments.get("city"))
        .and_then(|value| value.as_str())
        .unwrap_or("the requested city");
    Ok(GetPromptResult::new(vec![PromptMessage::new_text(
        Role::User,
        format!("Prepare a concise travel weather report for {city}."),
    )])
    .into())
}
