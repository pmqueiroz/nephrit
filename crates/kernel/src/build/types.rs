use bindings::transform::RegisteredTransform;

pub type TransformersCollection<'transforms> = Vec<&'transforms RegisteredTransform>;
