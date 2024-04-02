use actix::prelude::*;
use rand::{rngs::ThreadRng, Rng};

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {}

#[derive(Debug)]
pub struct GameServer {
    rng: ThreadRng,
}

impl GameServer {
    pub fn new() -> GameServer {
        Self {
            rng: rand::thread_rng()
        }
    }
}

impl Actor for GameServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

impl Handler<Connect> for GameServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");
        // register session with random id
        let id = self.rng.gen::<usize>();
        id
    }
}
