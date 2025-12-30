use bindings::token::ResolvedToken;

pub fn is_value_ref(value: &serde_json::Value) -> bool {
  match value {
    serde_json::Value::String(s) => s.starts_with('{') && s.ends_with('}'),
    _ => false,
  }
}

pub fn resolve_value_ref(
  value: &serde_json::Value,
  resolved_tokens: &std::collections::HashMap<String, ResolvedToken>,
) -> Option<serde_json::Value> {
  if let serde_json::Value::String(key) = value {
    if is_value_ref(value) {
      if let Some(token) = resolved_tokens.get(&key.to_string()) {
        return Some(token.value.clone());
      }
    }
  }
  None
}
