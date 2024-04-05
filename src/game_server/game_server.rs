use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use actix::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use crate::game_server::peer::ClientPosition;

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub peer_addr: Recipient<PeerPlayerData>,
}

#[derive(Message, Debug, Clone, Copy)]
#[rtype(result = "()")]
pub enum PeerPlayerData {
    PlayerJoined {
        player_id: usize
    },
    PlayerLeft {
        player_id: usize
    },
    PlayerPositionUpdate {
        player_position: ClientPosition,
        player_id: usize,
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PeerPlayerPositionUpdate {
    pub player_position: ClientPosition,
    pub player_id: usize,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Debug)]
pub struct GameServer {
    peer_addr_map: HashMap<usize, Recipient<PeerPlayerData>>,
    rng: ThreadRng,
    players_online_count: Arc<AtomicUsize>,
}

impl GameServer {
    pub fn new(players_online_count: Arc<AtomicUsize>) -> GameServer {
        Self {
            peer_addr_map: Default::default(),
            rng: rand::thread_rng(),
            players_online_count
        }
    }

    pub fn send_position_to_other_players(&self, data: PeerPlayerData, skip_id: usize) {
        for id in self.peer_addr_map.keys() {
            if *id != skip_id {
                if let Some(addr) = self.peer_addr_map.get(id) {
                    addr.do_send(data.clone());
                }
            }
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
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.peer_addr_map.insert(id, msg.peer_addr);

        // send message to other users
        self.send_position_to_other_players(PeerPlayerData::PlayerJoined {
            player_id: id
        }, id);

        self.players_online_count.fetch_add(1, Ordering::SeqCst);
        id
    }
}

impl Handler<Disconnect> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        println!("Someone disconnected!");
        // remove peer address
        if let _ = self.peer_addr_map.remove(&msg.id) {
            // send message to other users
            self.send_position_to_other_players(PeerPlayerData::PlayerLeft {
                player_id: msg.id
            }, 0);

            self.players_online_count.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

impl Handler<PeerPlayerPositionUpdate> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: PeerPlayerPositionUpdate, _: &mut Self::Context) -> Self::Result {
        let player_position_update = PeerPlayerData::PlayerPositionUpdate {
            player_position: msg.player_position,
            player_id: msg.player_id,
        };
        self.send_position_to_other_players(player_position_update, msg.player_id);
    }
}