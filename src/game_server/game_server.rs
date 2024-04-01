use actix::prelude::*;

#[derive(Debug)]
pub struct GameServer {}

impl GameServer {
    pub fn new() -> GameServer {
        Self {}
    }
}

impl Actor for GameServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}
