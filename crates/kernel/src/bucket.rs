use std::collections::HashMap;

#[derive(Debug)]
pub struct Token {
  pub path: String,
  pub value: serde_json::Value,
}

pub struct TokensBucket {
  pub resolved_tokens: HashMap<String, Token>,
  pub tokens_with_references: HashMap<String, Token>,
}

impl TokensBucket {
  pub fn new(raw_tokens: Vec<serde_json::Value>) -> Self {
    let mut resolved_tokens = HashMap::new();
    let mut tokens_with_references = HashMap::new();
    flatten(
      raw_tokens,
      "".into(),
      &mut resolved_tokens,
      &mut tokens_with_references,
    );

    Self {
      resolved_tokens,
      tokens_with_references,
    }
  }

  pub fn print_tokens(&self) {
    println!("Resolved Tokens: {:#?}", &self.resolved_tokens);
    println!(
      "Tokens with References: {:#?}",
      &self.tokens_with_references
    );
  }
}

fn flatten(
  values: Vec<serde_json::Value>,
  prefix: String,
  resolved_map: &mut HashMap<String, Token>,
  references_map: &mut HashMap<String, Token>,
) {
  for value in values {
    match value {
      serde_json::Value::Object(obj) => {
        for (key, val) in obj {
          let new_prefix = if prefix.is_empty() {
            key.clone()
          } else {
            format!("{}.{}", prefix, key)
          };
          flatten(vec![val], new_prefix, resolved_map, references_map);
        }
      }
      _ => {
        let token = Token {
          path: prefix.clone(),
          value: value.clone(),
        };

        if utils::is_value_ref(&value) {
          references_map.insert(prefix.clone(), token);
        } else {
          resolved_map.insert(prefix.clone(), token);
        }
      }
    }
  }
}
