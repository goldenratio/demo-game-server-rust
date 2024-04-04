use std::time::Instant;
use crate::game_server::game_server;
use actix::{Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, fut, Handler, Running, StreamHandler, WrapFuture};
use actix_web_actors::ws;

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
impl Handler<game_server::ConnectionMessage> for Peer {
    type Result = ();

    fn handle(&mut self, msg: game_server::ConnectionMessage, _ctx: &mut Self::Context) {
        println!("Peer {:?} - game_server::ConnectionMessage {:?}", self.id, msg);
        // ctx.text(msg.0);
    }
}

/// Handle messages from game server, we simply send it to peer websocket
impl Handler<game_server::GameObjectsInfo> for Peer {
    type Result = ();

    fn handle(&mut self, msg: game_server::GameObjectsInfo, _ctx: &mut Self::Context) {
        println!("Peer {:?} - game_server::GameObjectsInfo {:?}", self.id, msg);
        todo!();
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

        println!("websocket message {:?}", msg);

        match msg {
            ws::Message::Text(text) => {
                println!("received from client (text), {:?}", text);
            }
            ws::Message::Binary(bytes) => {
                println!("received from client (binary), {:?}", bytes);
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
