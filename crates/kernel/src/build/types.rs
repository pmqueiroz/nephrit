use bindings::transform::RegisteredTransform;

pub type TransformersCollection<'transforms> = Vec<&'transforms RegisteredTransform>;

pub struct Destination {
  pub path: std::path::PathBuf,
  pub name: String,
}
