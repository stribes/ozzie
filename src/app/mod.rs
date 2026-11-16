mod state;

use crate::{bot, env::Env, web};
pub use state::State;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn run(env: Env) -> Result<(), Error> {
    let state = State::new(env);

    tokio::try_join!(web::run(state.clone()), bot::run(state),)?;

    Ok(())
}
