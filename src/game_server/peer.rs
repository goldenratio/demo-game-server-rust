use crate::game_server::game_server::GameServer;
use actix::{Actor, ActorContext, Addr, StreamHandler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message;

pub struct Peer {
    /// unique session id
    pub id: i32,

    /// game server actor address
    pub game_server_addr: Addr<GameServer>,
}

impl Peer {}

impl Actor for Peer {
    type Context = ws::WebsocketContext<Self>;
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

        println!("websocket message {:?}", msg);

        match msg {
            Message::Text(_) => {}
            Message::Binary(bytes) => {
                println!("received from client, {:?}", bytes);
            }
            Message::Ping(_) => {}
            Message::Pong(_) => {}
            Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            Message::Continuation(_) => {
                ctx.stop();
            }
            Message::Nop => {}
        }
    }
}