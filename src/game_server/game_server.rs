use std::sync::atomic::{AtomicI32, Ordering};
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use flatbuffers::FlatBufferBuilder;
use crate::game_schema_generated::users::{finish_user_buffer, root_as_user, User, UserArgs};

// Define a global atomic variable to store the last generated ID
static NEXT_ID: AtomicI32 = AtomicI32::new(0);

fn generate_new_id() -> i32 {
    // Fetch the current value of the atomic variable and increment it atomically
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

/// Define HTTP actor
pub struct MyWs {
    id: i32,
}

impl MyWs {
    pub fn new() -> Self {
        Self {
            id: generate_new_id(),
        }
    }
}

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
                    "{} has id {}. The encoded data is {} bytes long. Actor id: {}",
                    name,
                    id,
                    bin.len(),
                    self.id
                );

                let mut bldr = FlatBufferBuilder::new();
                let mut bytes: Vec<u8> = Vec::new();
                make_user(&mut bldr, &mut bytes, name, id);

                ctx.binary(bytes)
            }
            _ => (),
        }
    }
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
