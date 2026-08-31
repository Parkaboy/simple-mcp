//! Additional MCP tools used to demonstrate composition and structured data.

use schemars::JsonSchema;
use serde::Deserialize;

/// Input for the forecast comparison tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CompareForecastsArgs {
    /// First city to compare.
    pub(crate) first_city: String,
    /// Second city to compare.
    pub(crate) second_city: String,
}

/// Validates the two city names used by the comparison tool.
pub(crate) fn compare_cities(args: &CompareForecastsArgs) -> Result<(&str, &str), String> {
    let first = crate::validation::city_name(&args.first_city)?;
    let second = crate::validation::city_name(&args.second_city)?;
    Ok((first, second))
}
