mod game_schema_generated;
mod route_handlers;
mod game_server;

use std::sync::Arc;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, HttpServer};
use std::sync::atomic::{AtomicUsize};
use crate::route_handlers::stats::get_stats;
use crate::route_handlers::create_ws::create_ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set up applications state
    // keep a count of the number of visitors
    let players_online = Arc::new(AtomicUsize::new(0));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(players_online.clone()))
            .route("/stats", web::get().to(get_stats))
            .route("/ws", web::get().to(create_ws))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}