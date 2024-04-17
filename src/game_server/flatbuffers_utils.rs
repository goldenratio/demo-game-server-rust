use flatbuffers::{FlatBufferBuilder};
use crate::game_schema_generated::gameplay_fbdata::{GameReponseEvent, GameReponseEventArgs, GameWorldUpdate, GameWorldUpdateArgs, PlayerControl, PlayerData, RemotePeerJoined, RemotePeerJoinedArgs, RemotePeerLeft, RemotePeerLeftArgs, RemotePeerPositionUpdate, RemotePeerPositionUpdateArgs, ResponseMessage, root_as_gameplay, Vec2};
use crate::game_server::game_world::PeerPlayerInfo;
use crate::game_server::peer::{ClientControls, ClientData, ClientPosition};

pub fn read_gameplay_data(buf: &[u8]) -> ClientData {
    let gameplay = root_as_gameplay(buf).unwrap();
    let player_controls = gameplay.player_controls().unwrap_or_else(|| &PlayerControl([0; 4]));
    let player_position = gameplay.player_position().unwrap_or_else(|| &Vec2([0; 8]));

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
    let player_position = Vec2::new(player_position.x, player_position.y);
    let player_data = PlayerData::new(player_id as u64, &player_position);

    let msg = RemotePeerPositionUpdate::create(&mut bldr, &RemotePeerPositionUpdateArgs {
        player_data: Option::from(&player_data)
    }).as_union_value();

    let args = GameReponseEventArgs {
        msg_type: ResponseMessage::RemotePeerPositionUpdate,
        msg: Option::from(msg)
    };

    // Call the `User::create` function with the `FlatBufferBuilder` and our
    // UserArgs object, to serialize the data to the FlatBuffer. The returned
    // value is an offset used to track the location of this serializaed data.
    let user_offset = GameReponseEvent::create(&mut bldr, &args);

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

    let msg = RemotePeerLeft::create(&mut bldr, &RemotePeerLeftArgs {
        player_id: player_id as u64
    }).as_union_value();

    let args = GameReponseEventArgs {
        msg_type: ResponseMessage::RemotePeerLeft,
        msg: Option::from(msg)
    };

    // Call the `User::create` function with the `FlatBufferBuilder` and our
    // UserArgs object, to serialize the data to the FlatBuffer. The returned
    // value is an offset used to track the location of this serializaed data.
    let user_offset = GameReponseEvent::create(&mut bldr, &args);

    // Finish the write operation by calling the generated function
    // `finish_user_buffer` with the `user_offset` created by `User::create`.
    bldr.finish(user_offset, None);

    // Copy the serialized FlatBuffers data to our own byte buffer.
    let finished_data = bldr.finished_data();
    bytes.extend_from_slice(finished_data);

    bytes
}

pub fn create_peer_joined_bytes(player_id: usize, player_position: ClientPosition) -> Vec<u8> {
    let mut bldr = FlatBufferBuilder::new();
    let mut bytes: Vec<u8> = Vec::new();

    // Reset the `bytes` Vec to a clean state.
    bytes.clear();

    // Reset the `FlatBufferBuilder` to a clean state.
    bldr.reset();

    let player_data = PlayerData::new(player_id as u64, &Vec2::new(player_position.x, player_position.y));

    let msg = RemotePeerJoined::create(&mut bldr, &RemotePeerJoinedArgs {
        player_data: Option::from(&player_data)
    }).as_union_value();

    let args = GameReponseEventArgs {
        msg_type: ResponseMessage::RemotePeerJoined,
        msg: Option::from(msg)
    };

    // Call the `User::create` function with the `FlatBufferBuilder` and our
    // UserArgs object, to serialize the data to the FlatBuffer. The returned
    // value is an offset used to track the location of this serializaed data.
    let user_offset = GameReponseEvent::create(&mut bldr, &args);

    // Finish the write operation by calling the generated function
    // `finish_user_buffer` with the `user_offset` created by `User::create`.
    bldr.finish(user_offset, None);

    // Copy the serialized FlatBuffers data to our own byte buffer.
    let finished_data = bldr.finished_data();
    bytes.extend_from_slice(finished_data);

    bytes
}

pub fn create_world_update_bytes(world_data: Vec<PeerPlayerInfo>) -> Vec<u8> {
    let mut bldr = FlatBufferBuilder::new();
    let mut bytes: Vec<u8> = Vec::new();

    // Reset the `bytes` Vec to a clean state.
    bytes.clear();

    // Reset the `FlatBufferBuilder` to a clean state.
    bldr.reset();

    // Create a temporary `UserArgs` object to build a `User` object.
    // (Note how we call `bldr.create_string` to create the UTF-8 string
    // ergonomically.)
    // let player_position = Vec2::new(player_position.x, player_position.y);
    // let player_data = PlayerData::new(player_id as u32, &player_position);

    let player_data_list = world_data.iter().map(|data| {
        let player_position = Vec2::new(data.x, data.y);
        let player_data = PlayerData::new(data.player_id as u64, &player_position);
        player_data
    }).collect::<Vec<PlayerData>>();

    let player_data_vec = bldr.create_vector(&player_data_list);

    let msg = GameWorldUpdate::create(&mut bldr, &GameWorldUpdateArgs {
        player_data_list: Option::from(player_data_vec)
    }).as_union_value();

    let args = GameReponseEventArgs {
        msg_type: ResponseMessage::GameWorldUpdate,
        msg: Option::from(msg)
    };

    // Call the `User::create` function with the `FlatBufferBuilder` and our
    // UserArgs object, to serialize the data to the FlatBuffer. The returned
    // value is an offset used to track the location of this serializaed data.
    let user_offset = GameReponseEvent::create(&mut bldr, &args);

    // Finish the write operation by calling the generated function
    // `finish_user_buffer` with the `user_offset` created by `User::create`.
    bldr.finish(user_offset, None);

    // Copy the serialized FlatBuffers data to our own byte buffer.
    let finished_data = bldr.finished_data();
    bytes.extend_from_slice(finished_data);

    bytes
}
