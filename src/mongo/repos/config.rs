use mongodb::{
    Collection, Database,
    bson::{doc, to_document},
};
use serde::Serialize;

use crate::models::Config;

#[derive(Clone)]
pub struct ConfigRepo {
    col: Collection<Config>,
}

impl ConfigRepo {
    pub fn new(db: &Database) -> Self {
        Self {
            col: db.collection("config"),
        }
    }

    pub async fn get(&self) -> mongodb::error::Result<Config> {
        Ok(self
            .col
            .find_one(doc! { "_id": "1" })
            .await?
            .unwrap_or_default())
    }

    pub async fn upsert(&self, config: &Config) -> mongodb::error::Result<()> {
        let mut values = to_document(config)?;
        values.remove("_id");

        self.update(values).await
    }

    pub async fn update<T: Serialize>(&self, values: T) -> mongodb::error::Result<()> {
        let values = to_document(&values)?;

        self.col
            .update_one(doc! { "_id": "1" }, doc! { "$set": values })
            .upsert(true)
            .await?;

        Ok(())
    }
}
