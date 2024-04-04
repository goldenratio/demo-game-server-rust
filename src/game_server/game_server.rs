use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use actix::prelude::*;
use rand::{rngs::ThreadRng, Rng};

/// Chat server sends this messages to session
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ConnectionMessage(pub String);

/// Chat server sends this messages to session
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct GameObjectsInfo {
    pub x: i32,
    pub y: i32,
}

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub peer_addr: Recipient<ConnectionMessage>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Debug)]
pub struct GameServer {
    rng: ThreadRng,
    players_online_count: Arc<AtomicUsize>,
}

impl GameServer {
    pub fn new(players_online_count: Arc<AtomicUsize>) -> GameServer {
        Self {
            rng: rand::thread_rng(),
            players_online_count
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

    /// triggered when an actor (peer) joined
    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // register session with random id
        let id = self.rng.gen::<usize>();

        // TODO store peer address

        msg.peer_addr.do_send(ConnectionMessage("hello world!".to_owned()));
        self.players_online_count.fetch_add(1, Ordering::SeqCst);
        id
    }
}

impl Handler<Disconnect> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        println!("Someone disconnected!");
        // remove peer address
        // TODO

        // send message to other users
        // TODO

        self.players_online_count.fetch_sub(1, Ordering::SeqCst);
    }
}