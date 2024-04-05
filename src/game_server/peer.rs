use std::time::Instant;
use crate::game_server::game_server;
use actix::{Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, fut, Handler, Running, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use flatbuffers::FlatBufferBuilder;
use crate::game_schema_generated::gameplay::{finish_gameplay_buffer, Gameplay, GameplayArgs, PlayerControl, PlayerPosition, root_as_gameplay};
use crate::game_server::game_server::PeerPlayerData;

#[derive(Debug)]
pub struct ClientControls {
    up: bool,
    down: bool,
    left: bool,
    right: bool
}

#[derive(Debug, Clone, Copy)]
pub struct ClientPosition {
    x: f32,
    y: f32
}

#[derive(Debug)]
pub struct ClientData {
    player_controls: ClientControls,
    player_position: ClientPosition
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

        let mut bldr = FlatBufferBuilder::new();
        let mut bytes: Vec<u8> = Vec::new();

        // Reset the `bytes` Vec to a clean state.
        bytes.clear();

        // Reset the `FlatBufferBuilder` to a clean state.
        bldr.reset();

        match msg {
            PeerPlayerData::PlayerJoined { player_id } => {
                //
            }
            PeerPlayerData::PlayerLeft { player_id } => {
                //
            }
            PeerPlayerData::PlayerPositionUpdate { player_id, player_position } => {
                // Create a temporary `UserArgs` object to build a `User` object.
                // (Note how we call `bldr.create_string` to create the UTF-8 string
                // ergonomically.)
                let player_position = PlayerPosition::new(player_position.x, player_position.y);

                let args = GameplayArgs {
                    player_id: Some(bldr.create_string(&*player_id.to_string())),
                    player_position: Option::from(&player_position),
                    player_controls: None
                };

                // Call the `User::create` function with the `FlatBufferBuilder` and our
                // UserArgs object, to serialize the data to the FlatBuffer. The returned
                // value is an offset used to track the location of this serializaed data.
                let user_offset = Gameplay::create(&mut bldr, &args);

                // Finish the write operation by calling the generated function
                // `finish_user_buffer` with the `user_offset` created by `User::create`.
                finish_gameplay_buffer(&mut bldr, user_offset);

                // Copy the serialized FlatBuffers data to our own byte buffer.
                let finished_data = bldr.finished_data();
                bytes.extend_from_slice(finished_data);

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

pub fn read_gameplay_data(buf: &[u8]) -> ClientData {
    let gameplay = root_as_gameplay(buf).unwrap();
    let player_controls = gameplay.player_controls().unwrap_or_else(|| &PlayerControl([0; 4]));
    let player_position = gameplay.player_position().unwrap_or_else(|| &PlayerPosition([0; 8]));

    ClientData {
        player_position: ClientPosition {
            x: player_position.x(),
            y: player_position.y(),
        },
        player_controls: ClientControls {
            up: player_controls.up(),
            down: player_controls.down(),
            left: player_controls.left(),
            right: player_controls.right(),
        }
    }
}
