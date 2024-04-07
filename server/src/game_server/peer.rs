use std::time::Instant;
use crate::game_server::game_server;
use actix::{Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, fut, Handler, Running, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use crate::game_server::flatbuffers_utils::{create_peer_left_bytes, create_peer_position_bytes, read_gameplay_data};

#[derive(Debug)]
pub struct ClientControls {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool
}

#[derive(Debug, Clone, Copy)]
pub struct ClientPosition {
    pub x: f32,
    pub y: f32
}

#[derive(Debug)]
pub struct ClientData {
    pub player_controls: ClientControls,
    pub player_position: ClientPosition
}

pub struct Peer {
    /// unique session id
    /// id is assigned when connection is established
    pub id: usize,

    pub heart_beat: Instant,

    /// game server actor address
    pub game_server_addr: Addr<game_server::GameServer>,
}

impl Peer {}

impl Actor for Peer {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let peer_addr = ctx.address();
        self.game_server_addr
            .send(game_server::Connect { peer_addr: peer_addr.recipient() })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => {
                        act.id = res;
                    },
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .then(|_, act,_| {
                println!("actor connected! id: {:?}", act.id);
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        // notify game server
        self.game_server_addr.do_send(game_server::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from game server, we simply send it to peer websocket
impl Handler<game_server::PeerPlayerData> for Peer {
    type Result = ();

    fn handle(&mut self, msg: game_server::PeerPlayerData, ctx: &mut Self::Context) {
        // println!("Peer {:?} - game_server::PeerPlayerData {:?}", self.id, msg);

        match msg {
            game_server::PeerPlayerData::RemotePeerJoined { player_id: player_id } => {
                //
            }
            game_server::PeerPlayerData::RemotePeerLeft { player_id: player_id } => {
                let bytes = create_peer_left_bytes(player_id);
                ctx.binary(bytes);
            }
            game_server::PeerPlayerData::RemotePeerPositionUpdate { player_id: player_id, player_position } => {
                let bytes = create_peer_position_bytes(player_id, player_position);
                ctx.binary(bytes);
            }
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Peer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            },
            Ok(msg) => msg,
        };

        // println!("websocket message {:?}", msg);

        match msg {
            ws::Message::Text(text) => {
                println!("received from client (text), {:?}", text);
            }
            ws::Message::Binary(bytes) => {
                let gameplay_data = read_gameplay_data(&bytes);
                // println!("received from client (binary), {:?} : {:?}", self.id, gameplay_data.player_position);
                self.game_server_addr.do_send(game_server::PeerPlayerPositionUpdate {
                    player_position: gameplay_data.player_position,
                    player_id: self.id
                });
            }
            ws::Message::Ping(msg) => {
                self.heart_beat = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.heart_beat = Instant::now();
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => {}
        }
    }
}
