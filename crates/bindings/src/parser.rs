use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct TokenFile {
  pub path: String,
  pub content: String,
}

#[derive(Clone, Debug)]
pub struct ParsedFile {
  pub path: String,
  pub content: serde_json::Value,
}

#[napi(object)]
#[derive(Clone)]
pub struct Parser<'parser> {
  pub name: String,
  pub pattern: String,
  pub parser: Function<'parser, TokenFile, String>,
}

pub struct RegisteredParser {
  pub name: String,
  pub pattern: String,
  pub parser: FunctionRef<TokenFile, String>,
}
