//! MCP resources for reference data and dynamic forecast context.

use rmcp::model::{
    ListResourceTemplatesResult, ListResourcesResult, PaginatedRequestParams,
    ReadResourceRequestParams, ReadResourceResponse, ReadResourceResult, Resource,
    ResourceContents, ResourceTemplate,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer};

/// Lists the static resources available to MCP clients.
pub(crate) async fn list_resources(
    _server: &crate::WeatherServer,
    _request: Option<PaginatedRequestParams>,
    _context: RequestContext<RoleServer>,
) -> Result<ListResourcesResult, ErrorData> {
    Ok(ListResourcesResult::with_all_items(vec![
        Resource::new("weather://supported-cities", "supported-cities")
            .with_description("Cities useful for testing this server")
            .with_mime_type("text/plain"),
    ]))
}

/// Lists the URI template for city-specific weather context.
pub(crate) async fn list_resource_templates(
    _server: &crate::WeatherServer,
    _request: Option<PaginatedRequestParams>,
    _context: RequestContext<RoleServer>,
) -> Result<ListResourceTemplatesResult, ErrorData> {
    Ok(ListResourceTemplatesResult::with_all_items(vec![
        ResourceTemplate::new("weather://forecast/{city}", "forecast-by-city")
            .with_description("Current forecast context for a city")
            .with_mime_type("application/json"),
    ]))
}

/// Reads the supported-city list or a city-specific resource template URI.
pub(crate) async fn read_resource(
    _server: &crate::WeatherServer,
    request: ReadResourceRequestParams,
    _context: RequestContext<RoleServer>,
) -> Result<ReadResourceResponse, ErrorData> {
    let uri = request.uri;
    let contents = match uri.as_str() {
        "weather://supported-cities" => {
            ResourceContents::text("Punta del Este\nMontevideo\nBuenos Aires", uri.clone())
        }
        uri if uri.starts_with("weather://forecast/") => ResourceContents::text(
            serde_json::json!({
                "city": uri.trim_start_matches("weather://forecast/"),
                "note": "Use get_forecast for live data"
            })
            .to_string(),
            uri,
        )
        .with_mime_type("application/json"),
        _ => return Err(ErrorData::resource_not_found("resource not found", None)),
    };
    Ok(ReadResourceResult::new(vec![contents]).into())
}
