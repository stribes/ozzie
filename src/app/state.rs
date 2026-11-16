use crate::env::Env;

pub struct State {
    pub env: Env,
}

impl State {
    pub fn new(env: Env) -> Self {
        Self { env }
    }
}
