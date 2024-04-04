use std::time::Instant;
use crate::game_server::game_server::GameServer;
use crate::game_server::peer::Peer;
use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub async fn create_ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<GameServer>>,
) -> Result<HttpResponse, Error> {
    let game_server_addr = srv.get_ref().clone();
    ws::start(
        Peer {
            id: 0,
            heart_beat: Instant::now(),
            game_server_addr,
        },
        &req,
        stream,
    )
}
