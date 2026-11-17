use mongodb::{Collection, Database, IndexModel, bson::doc, options::IndexOptions};

use crate::models::Guild;

#[derive(Clone)]
pub struct GuildsRepo {
    col: Collection<Guild>,
}

impl GuildsRepo {
    pub fn new(db: &Database) -> Self {
        Self {
            col: db.collection("guilds"),
        }
    }

    pub async fn ensure_indexes(&self) -> mongodb::error::Result<()> {
        let index = IndexModel::builder()
            .keys(doc! { "guild_id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        self.col.create_index(index).await?;

        Ok(())
    }

    pub async fn upsert(&self, guild: &Guild) -> mongodb::error::Result<()> {
        self.col
            .replace_one(doc! { "guild_id": guild.guild_id }, guild)
            .upsert(true)
            .await?;

        Ok(())
    }

    pub async fn get(&self, guild_id: i64) -> mongodb::error::Result<Option<Guild>> {
        self.col.find_one(doc! { "guild_id": guild_id }).await
    }

    pub async fn exists(&self, guild_id: i64) -> mongodb::error::Result<bool> {
        let count = self
            .col
            .count_documents(doc! { "guild_id": guild_id })
            .limit(1)
            .await?;

        Ok(count > 0)
    }

    pub async fn delete(&self, guild_id: i64) -> mongodb::error::Result<bool> {
        let result = self.col.delete_one(doc! { "guild_id": guild_id }).await?;

        Ok(result.deleted_count > 0)
    }
}
