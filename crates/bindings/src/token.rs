use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct Token {
  #[napi(js_name = "type")]
  pub _type: String,
  pub value: String,
}
#[napi(object)]
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolvedToken {
  pub path: String,
  #[napi(ts_type = "Token")]
  pub original_value: serde_json::Value,
  pub name: String,
  pub value: serde_json::Value,
}

#[napi(object)]
#[derive(Debug, Clone, serde::Serialize)]
pub struct TransformedToken {
  pub original: ResolvedToken,
  pub value: String,
  pub name: String,
}

impl TransformedToken {
  pub fn from_resolved_token(resolved_token: &ResolvedToken) -> Self {
    let formatted_value = match &resolved_token.value {
      serde_json::Value::String(s) => s.clone(),
      _ => resolved_token.value.to_string(),
    };

    Self {
      value: formatted_value,
      name: resolved_token.name.clone(),
      original: resolved_token.clone(),
    }
  }
}
