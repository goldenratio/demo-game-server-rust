use crate::game_server::game_server;
use actix::{Actor, ActorContext, ActorFutureExt, Addr, ContextFutureSpawner, fut, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use actix_web_actors::ws::Message;

pub struct Peer {
    /// unique session id
    /// id s assigned when connection is established
    pub id: usize,

    /// game server actor address
    pub game_server_addr: Addr<game_server::GameServer>,
}

impl Peer {}

impl Actor for Peer {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // println!("actor started! id: {:?}", ctx)
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        // let addr = ctx.address();
        self.game_server_addr
            .send(game_server::Connect {})
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
