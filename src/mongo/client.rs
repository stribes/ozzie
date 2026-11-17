use std::time::Duration;

use mongodb::{Client, Database, bson::doc, options::ClientOptions};

use super::repos::GuildsRepo;

#[derive(Clone)]
pub struct Mongo {
    pub client: Client,
    pub db: Database,
    pub guilds: GuildsRepo,
}

impl Mongo {
    pub async fn connect(uri: &str, db_name: &str) -> mongodb::error::Result<Self> {
        let mut options = ClientOptions::parse(uri).await?;
        options.app_name = Some("ozzie".into());
        options.server_selection_timeout = Some(Duration::from_secs(5));

        let client = Client::with_options(options)?;
        let db = client.database(db_name);

        db.run_command(doc! { "ping": 1 }).await?;
        tracing::info!(db = db_name, "connected to mongodb");

        let mongo = Self {
            guilds: GuildsRepo::new(&db),
            client,
            db,
        };

        mongo.ensure_indexes().await?;
        Ok(mongo)
    }

    async fn ensure_indexes(&self) -> mongodb::error::Result<()> {
        self.guilds.ensure_indexes().await?;
        Ok(())
    }
}
