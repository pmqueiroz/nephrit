use super::types::TransformersCollection;
use bindings::{RegisteredTransforms, TransformGroup};
use log::Logger;

pub fn resolve_transformers<'transforms>(
  transform_group: TransformGroup,
  transforms: &'transforms RegisteredTransforms,
) -> TransformersCollection<'transforms> {
  let mut collection = TransformersCollection::new();

  for group_transform in transform_group.transforms {
    if let Some(transform) = transforms.get(&group_transform) {
      collection.push(transform);
    } else {
      Logger::error(&format!(
        "Transform '{}' not found in the registered transforms.",
        group_transform
      ));
      std::process::exit(1);
    }
  }

  collection
}
