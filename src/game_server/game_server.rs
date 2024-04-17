use std::collections::{HashMap};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use actix::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use crate::game_server::game_world::GameWorld;
use crate::game_server::message_types::{Connect, Disconnect, PeerPlayerData, PeerPlayerPositionUpdate};
use crate::game_server::peer::ClientPosition;

#[derive(Debug)]
pub struct GameServer {
    peer_addr_map: HashMap<usize, Recipient<PeerPlayerData>>,
    rng: ThreadRng,
    players_online_count: Arc<AtomicUsize>,
    game_world: GameWorld
}

impl GameServer {
    pub fn new(players_online_count: Arc<AtomicUsize>) -> GameServer {
        Self {
            peer_addr_map: Default::default(),
            rng: rand::thread_rng(),
            players_online_count,
            game_world: Default::default()
        }
    }

    pub fn send_position_to_other_players(&self, data: PeerPlayerData, skip_id: Option<usize>) {
        let skip_id_value = skip_id.unwrap_or_else(|| 0);
        for id in self.peer_addr_map.keys() {
            if *id != skip_id_value {
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

        self.game_world.add_player(id);

        // send message to other users
        self.send_position_to_other_players(PeerPlayerData::RemotePeerJoined {
            player_id: id,
            player_position: ClientPosition { x: 10.0, y: 10.0 }
        }, Option::from(id));

        // send world update to current peer
        let world_data = self.game_world.get_world_update(id);
        if let Some(addr) = self.peer_addr_map.get(&id) {
            addr.do_send(PeerPlayerData::WorldUpdate { world_data });
        }

        self.players_online_count.fetch_add(1, Ordering::SeqCst);
        id
    }
}

impl Handler<Disconnect> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        println!("Someone disconnected!");
        // remove peer address
        if let Some(_) = self.peer_addr_map.remove(&msg.id) {
            // send message to other users
            self.send_position_to_other_players(PeerPlayerData::RemotePeerLeft {
                player_id: msg.id
            }, None);

            self.game_world.remove_player(msg.id);
            self.players_online_count.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

impl Handler<PeerPlayerPositionUpdate> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: PeerPlayerPositionUpdate, _: &mut Self::Context) -> Self::Result {
        let player_position_update = PeerPlayerData::RemotePeerPositionUpdate {
            player_position: msg.player_position,
            player_id: msg.player_id,
        };
        self.game_world.update_player_position(msg.player_id, msg.player_position.x, msg.player_position.y);
        self.send_position_to_other_players(player_position_update, Option::from(msg.player_id));
    }
}
