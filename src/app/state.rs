use crate::{env::Env, mongo::Mongo};

#[derive(Clone)]
pub struct State {
    pub env: Env,
    pub mongo: Mongo,
}

impl State {
    pub fn new(env: Env, mongo: Mongo) -> Self {
        Self { env, mongo }
    }
}
