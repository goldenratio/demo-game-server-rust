mod game_schema_generated;

use crate::game_schema_generated::users::root_as_user;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use flatbuffers::FlatBufferBuilder;
use game_schema_generated::users::{finish_user_buffer, User, UserArgs};

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("{:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => {
                let (name, id) = read_user(&bin[..]);
                // Show the decoded information:
                println!(
                    "{} has id {}. The encoded data is {} bytes long.",
                    name,
                    id,
                    bin.len()
                );

                ctx.binary(bin)
            }
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut bldr = FlatBufferBuilder::new();
    let mut bytes: Vec<u8> = Vec::new();

    // Write the provided `name` and `id` into the `bytes` Vec using the
    // FlatBufferBuilder `bldr`:
    make_user(&mut bldr, &mut bytes, "Arthur Dent", 42);

    // Now, `bytes` contains the serialized representation of our User object.

    // To read the serialized data, call our `read_user` function to decode
    // the `user` and `id`:
    println!("{:?}", bytes);
    let (name, id) = read_user(&bytes);

    // Show the decoded information:
    println!(
        "{} has id {}. The encoded data is {} bytes long.",
        name,
        id,
        bytes.len()
    );

    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

pub fn make_user(bldr: &mut FlatBufferBuilder, dest: &mut Vec<u8>, name: &str, id: u64) {
    // Reset the `bytes` Vec to a clean state.
    dest.clear();

    // Reset the `FlatBufferBuilder` to a clean state.
    bldr.reset();

    // Create a temporary `UserArgs` object to build a `User` object.
    // (Note how we call `bldr.create_string` to create the UTF-8 string
    // ergonomically.)
    let args = UserArgs {
        name: Some(bldr.create_string(name)),
        id: id,
    };

    // Call the `User::create` function with the `FlatBufferBuilder` and our
    // UserArgs object, to serialize the data to the FlatBuffer. The returned
    // value is an offset used to track the location of this serializaed data.
    let user_offset = User::create(bldr, &args);

    // Finish the write operation by calling the generated function
    // `finish_user_buffer` with the `user_offset` created by `User::create`.
    finish_user_buffer(bldr, user_offset);

    // Copy the serialized FlatBuffers data to our own byte buffer.
    let finished_data = bldr.finished_data();
    dest.extend_from_slice(finished_data);
}

pub fn read_user(buf: &[u8]) -> (&str, u64) {
    let u = root_as_user(buf).unwrap();
    let name = u.name().unwrap();
    let id = u.id();
    (name, id)
}
