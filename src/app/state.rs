use crate::env::Env;

#[derive(Clone)]
pub struct State {
    pub env: Env,
}

impl State {
    pub fn new(env: Env) -> Self {
        Self { env }
    }
}
