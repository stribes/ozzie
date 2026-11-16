pub mod state;

use crate::env::Env;
use state::State;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn run(env: Env) -> Result<(), Error> {
    let _state = State::new(env);

    // tokio::try_join!()

    Ok(())
}
