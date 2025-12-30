use bindings::parser::ParsedFile;
use serde_json::{Map, Value};

pub fn merge_tokens(files: Vec<ParsedFile>) -> Value {
  let mut merged = Map::new();

  for file in files {
    if let Value::Object(obj) = file.content {
      merged.extend(obj);
    }
  }

  Value::Object(merged)
}
