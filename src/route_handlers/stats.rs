use std::sync::atomic::{AtomicUsize, Ordering};
use actix_web::{Responder, web};
use serde::{Serialize};

#[derive(Debug, Serialize)]
struct StatsResponseData {
    #[serde(rename = "playersOnline")]
    players_online: usize,
}

/// Displays state
pub async fn get_stats(count: web::Data<AtomicUsize>) -> impl Responder {
    let players_online = count.load(Ordering::SeqCst);
    let response_data = StatsResponseData {
        players_online
    };
    return web::Json(response_data);
}
