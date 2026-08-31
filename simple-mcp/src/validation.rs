//! Input validation for the weather tools.
//!
//! Keeping validation separate makes the rules easy to test and explain.

/// Validates and trims a city name supplied by an MCP client.
pub(crate) fn city_name(input: &str) -> Result<&str, String> {
    let city = input.trim();
    if city.is_empty() {
        return Err("city must not be empty".to_string());
    }
    if city.chars().count() > 100 {
        return Err("city must be 100 characters or fewer".to_string());
    }
    Ok(city)
}

#[cfg(test)]
mod tests {
    use super::city_name;

    /// Accepts a normal city and removes surrounding whitespace.
    #[test]
    fn trims_city_name() {
        assert_eq!(city_name(" Punta del Este ").unwrap(), "Punta del Este");
    }

    /// Rejects blank input before any network request is made.
    #[test]
    fn rejects_blank_city() {
        assert!(city_name(" \t").is_err());
    }
}
