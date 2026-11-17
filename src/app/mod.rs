mod state;

use crate::{bot, env::Env, mongo::Mongo, web};
pub use state::State;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn run(env: Env) -> Result<(), Error> {
    let mongo = Mongo::connect(&env.MONGODB_URL, &env.MONGODB_NAME).await?;
    let state = State::new(env, mongo);

    tokio::try_join!(web::run(state.clone()), bot::run(state),)?;

    Ok(())
}
