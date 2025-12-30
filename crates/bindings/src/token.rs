use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct OriginalToken {
  #[napi(js_name = "type")]
  pub _type: String,
  pub value: String,
}

#[derive(Debug, Clone)]
pub struct ResolvedToken {
  pub key: String,
  pub original_value: serde_json::Value,
  pub name: String,
  pub value: serde_json::Value,
  pub path: Vec<String>,
  pub file_path: String,
}

#[napi(object)]
#[derive(Debug, Clone, serde::Serialize)]
pub struct TokenAttrs {
  pub category: Option<String>,
  #[napi(js_name = "type")]
  pub _type: Option<String>,
  pub item: Option<String>,
  pub subitem: Option<String>,
  pub state: Option<String>,
}

impl TokenAttrs {
  pub fn from_path(path: &Vec<String>) -> Self {
    Self {
      category: path.get(0).cloned(),
      _type: path.get(1).cloned(),
      item: path.get(2).cloned(),
      subitem: path.get(3).cloned(),
      state: path.get(4).cloned(),
    }
  }
}

#[napi(object)]
#[derive(Debug, Clone, serde::Serialize)]
pub struct TransformedToken {
  pub key: String,
  pub value: String,
  pub file_path: String,
  pub is_source: bool,
  #[napi(ts_type = "OriginalToken & { [k: string]: any }")]
  pub original: serde_json::Value,
  pub name: String,
  pub attributes: TokenAttrs,
  pub path: Vec<String>,
}

impl TransformedToken {
  pub fn from_resolved_token(resolved_token: &ResolvedToken) -> Self {
    let formatted_value = match &resolved_token.value {
      serde_json::Value::String(s) => s.clone(),
      _ => resolved_token.value.to_string(),
    };

    Self {
      key: resolved_token.key.clone(),
      value: formatted_value,
      file_path: resolved_token.file_path.clone(),
      is_source: true,
      original: resolved_token.original_value.clone(),
      name: resolved_token.name.clone(),
      attributes: TokenAttrs::from_path(&resolved_token.path),
      path: resolved_token.path.clone(),
    }
  }
}
