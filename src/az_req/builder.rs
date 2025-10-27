// src/az_req/builder.rs

use serde_json::Value;
use std::collections::HashMap;

/// Performs a deep copy of a JSON-like `HashMap<String, Value>`.
/// Returns an empty map if `None` is provided.
pub fn deep_copy(source: Option<&HashMap<String, Value>>) -> HashMap<String, Value> {
    match source {
        Some(map) => map.clone(), // Clones keys and values (cheap for small maps)
        None => HashMap::new(),
    }
}
