use actix_web::{web, Responder};
use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Serialize)]
struct StatsResponseData {
    #[serde(rename = "playersOnline")]
    players_online: usize,
}

/// Displays state
pub async fn get_stats(count: web::Data<AtomicUsize>) -> impl Responder {
    let players_online = count.load(Ordering::SeqCst);
    let response_data = StatsResponseData { players_online };
    return web::Json(response_data);
}
