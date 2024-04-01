mod game_schema_generated;
mod game_server;
mod route_handlers;

use crate::game_server::game_server::GameServer;
use crate::route_handlers::create_ws::create_ws;
use crate::route_handlers::stats::get_stats;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, HttpServer};
use log::info;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // set up applications state
    // keep a count of the number of visitors
    let players_online = Arc::new(AtomicUsize::new(0));

    let game_server = GameServer::new().start();

    info!("running server in port 8090");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(players_online.clone()))
            .app_data(web::Data::new(game_server.clone()))
            .route("/stats", web::get().to(get_stats))
            .route("/ws", web::get().to(create_ws))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
