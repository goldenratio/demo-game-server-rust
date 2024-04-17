use actix::{Message, Recipient};
use crate::game_server::game_world::PeerPlayerInfo;
use crate::game_server::peer::ClientPosition;

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub peer_addr: Recipient<PeerPlayerData>,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub enum PeerPlayerData {
    RemotePeerJoined {
        player_id: usize,
        player_position: ClientPosition,
    },
    RemotePeerLeft {
        player_id: usize
    },
    RemotePeerPositionUpdate {
        player_id: usize,
        player_position: ClientPosition,
    },
    WorldUpdate {
        world_data: Vec<PeerPlayerInfo>,
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PeerPlayerPositionUpdate {
    pub player_position: ClientPosition,
    pub player_id: usize,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}
