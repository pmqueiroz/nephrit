use crate::build::types::Destination;

pub fn get_file_path(
  cwd: &Option<std::path::PathBuf>,
  built_path: String,
  destination: String,
) -> Destination {
  let name = format!("{}/{}", built_path, destination);

  let path = if let Some(cwd_path) = cwd {
    cwd_path.join(&built_path).join(&destination)
  } else {
    std::path::Path::new(&built_path).join(&destination)
  };

  Destination { path, name }
}
