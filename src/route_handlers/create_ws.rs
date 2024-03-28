use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use crate::game_server::game_server::MyWs;

pub async fn create_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs::new(), &req, stream);
    println!("{:?}", resp);
    resp
}