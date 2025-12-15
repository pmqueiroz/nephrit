pub fn is_value_ref(value: &serde_json::Value) -> bool {
  match value {
    serde_json::Value::String(s) => s.starts_with('{') && s.ends_with('}'),
    _ => false,
  }
}
