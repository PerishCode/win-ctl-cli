use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Profile {
    #[serde(default)]
    pub quiet: Option<bool>,

    #[serde(default)]
    pub verbose: Option<u8>,

    #[serde(default)]
    pub log_level: Option<String>,
}
