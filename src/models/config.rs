use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "_id")]
    pub id: String,
    pub prefix: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            id: "1".to_string(),
            prefix: ".".to_string(),
        }
    }
}
