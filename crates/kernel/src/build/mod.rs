use std::collections::HashMap;

use bindings::{platform::Platform, token::TransformedToken};
use log::Logger;
use napi::bindgen_prelude::Env;

mod resolve_transformers;
pub mod types;

use crate::TokensBucket;

pub use self::resolve_transformers::resolve_transformers;

pub fn build<'transforms>(
  platform: Platform,
  collection: types::TransformersCollection<'transforms>,
  bucket: &TokensBucket,
  env: &Env,
) {
  Logger::debug(&format!("Building platform: {}", platform.name));
  let mut transformed_tokens = HashMap::new();

  for transformer in collection {
    Logger::info(&format!("Applying transformer: {}", transformer.name));
    for token in bucket.iter() {
      if let Ok(filter_func) = transformer.filter.borrow_back(env) {
        let token_json = serde_json::to_value(token).unwrap_or(serde_json::Value::Null);
        let bool_result = filter_func.call(token_json.clone());
        if let Ok(boolean) = bool_result {
          if boolean {
            Logger::debug(&format!(
              "Transformer '{}' matched token: {}",
              transformer.name, token.path
            ));
            if let Ok(transform_func) = transformer.transform.borrow_back(env) {
              let transformed_result = transform_func.call(token_json);
              match transformed_result {
                Ok(transformed_code) => {
                  transformed_tokens.insert(
                    token.path.clone(),
                    TransformedToken {
                      original: token.clone(),
                      value: transformed_code,
                    },
                  );
                }
                Err(e) => {
                  Logger::error(&format!(
                    "Error transforming token {:?} with transformer '{}': {:?}",
                    token, transformer.name, e
                  ));
                }
              }
            } else {
              Logger::error(&format!(
                "Failed to borrow transform function for transformer '{}'",
                transformer.name
              ));
            }
          }
        }
      }
    }
  }

  Logger::info(&format!(
    "Finished building platform: {}. Transformed {} tokens.",
    platform.name,
    transformed_tokens.len()
  ));
}
