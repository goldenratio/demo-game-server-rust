use flatbuffers::FlatBufferBuilder;
use crate::game_schema_generated::gameplay_fbdata::{GameEvent, GameEventArgs, GameEventType, Gameplay, GameplayArgs, PlayerControl, PlayerPosition, root_as_gameplay};
use crate::game_server::peer::{ClientControls, ClientData, ClientPosition};

pub fn read_gameplay_data(buf: &[u8]) -> ClientData {
    let gameplay = root_as_gameplay(buf).unwrap();
    let player_controls = gameplay.player_controls().unwrap_or_else(|| &PlayerControl([0; 4]));
    let player_position = gameplay.player_position().unwrap_or_else(|| &PlayerPosition([0; 8]));

    ClientData {
        player_position: ClientPosition {
            x: player_position.x(),
            y: player_position.y(),
        },
        player_controls: ClientControls {
            up: player_controls.up(),
            down: player_controls.down(),
            left: player_controls.left(),
            right: player_controls.right(),
        }
    }
}

pub fn create_peer_position_bytes(player_id: usize, player_position: ClientPosition) -> Vec<u8> {
    let mut bldr = FlatBufferBuilder::new();
    let mut bytes: Vec<u8> = Vec::new();

    // Reset the `bytes` Vec to a clean state.
    bytes.clear();

    // Reset the `FlatBufferBuilder` to a clean state.
    bldr.reset();

    // Create a temporary `UserArgs` object to build a `User` object.
    // (Note how we call `bldr.create_string` to create the UTF-8 string
    // ergonomically.)
    let player_position = PlayerPosition::new(player_position.x, player_position.y);

    let args = GameEventArgs {
        event_type: GameEventType::RemotePeerPositionUpdate,
        player_id: Some(bldr.create_string(&*player_id.to_string())),
        player_position: Option::from(&player_position),
    };

    // Call the `User::create` function with the `FlatBufferBuilder` and our
    // UserArgs object, to serialize the data to the FlatBuffer. The returned
    // value is an offset used to track the location of this serializaed data.
    let user_offset = GameEvent::create(&mut bldr, &args);

    // Finish the write operation by calling the generated function
    // `finish_user_buffer` with the `user_offset` created by `User::create`.
    bldr.finish(user_offset, None);

    // Copy the serialized FlatBuffers data to our own byte buffer.
    let finished_data = bldr.finished_data();
    bytes.extend_from_slice(finished_data);

    bytes
}

pub fn create_peer_left_bytes(player_id: usize) -> Vec<u8> {
    let mut bldr = FlatBufferBuilder::new();
    let mut bytes: Vec<u8> = Vec::new();

    // Reset the `bytes` Vec to a clean state.
    bytes.clear();

    // Reset the `FlatBufferBuilder` to a clean state.
    bldr.reset();

    let args = GameEventArgs {
        event_type: GameEventType::RemotePeerLeft,
        player_id: Some(bldr.create_string(&*player_id.to_string())),
        player_position: None
    };

    // Call the `User::create` function with the `FlatBufferBuilder` and our
    // UserArgs object, to serialize the data to the FlatBuffer. The returned
    // value is an offset used to track the location of this serializaed data.
    let user_offset = GameEvent::create(&mut bldr, &args);

    // Finish the write operation by calling the generated function
    // `finish_user_buffer` with the `user_offset` created by `User::create`.
    bldr.finish(user_offset, None);

    // Copy the serialized FlatBuffers data to our own byte buffer.
    let finished_data = bldr.finished_data();
    bytes.extend_from_slice(finished_data);

    bytes
}