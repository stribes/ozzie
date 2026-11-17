use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    pub guild_id: i64,
    pub guild_name: String,
}

impl Guild {
    pub fn new(guild_id: i64, guild_name: String) -> Self {
        Self {
            guild_id,
            guild_name,
        }
    }
}
