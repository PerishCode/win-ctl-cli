mod model;

pub use model::Profile;

use std::path::PathBuf;

pub fn load(path: Option<PathBuf>) -> Result<Profile, String> {
    let path = match path {
        Some(path) => path,
        None => return Ok(Profile::default()),
    };

    let contents = std::fs::read_to_string(&path)
        .map_err(|err| format!("failed to read profile {}: {err}", path.display()))?;
    serde_json::from_str(&contents)
        .map_err(|err| format!("failed to parse profile {}: {err}", path.display()))
}
