pub fn generate_name_from_key(path: &str) -> String {
  path.replace(".", "-")
}
