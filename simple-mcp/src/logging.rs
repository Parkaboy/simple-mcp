//! Minimal stderr logging for an MCP stdio process.
//!
//! MCP reserves stdout for JSON-RPC messages, so diagnostics belong on stderr.

/// Logs the start of a tool request without contaminating MCP stdout.
pub(crate) fn tool_started(tool: &str, city: &str) {
    eprintln!("tool={tool} city={city:?} event=start");
}

/// Logs a successful tool request.
pub(crate) fn tool_succeeded(tool: &str, city: &str) {
    eprintln!("tool={tool} city={city:?} event=success");
}

/// Logs a failed tool request and its client-safe error message.
pub(crate) fn tool_failed(tool: &str, city: &str, error: &str) {
    eprintln!("tool={tool} city={city:?} event=error message={error:?}");
}
