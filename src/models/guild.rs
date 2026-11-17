use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    pub guild_id: i64,
    #[serde(default)]
    pub prefix: String,
}

impl Guild {
    pub fn new(guild_id: i64) -> Self {
        Self {
            guild_id,
            prefix: String::new(),
        }
    }
}
