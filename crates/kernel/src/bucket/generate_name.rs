pub fn generate_name_from_path(path: &str) -> String {
  path.replace(".", "-")
}
