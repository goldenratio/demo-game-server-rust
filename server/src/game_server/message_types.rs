use actix::{Message, Recipient};
use crate::game_server::peer::ClientPosition;

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub peer_addr: Recipient<PeerPlayerData>,
}

#[derive(Message, Debug, Clone, Copy)]
#[rtype(result = "()")]
pub enum PeerPlayerData {
    RemotePeerJoined {
        player_id: usize
    },
    RemotePeerLeft {
        player_id: usize
    },
    RemotePeerPositionUpdate {
        player_position: ClientPosition,
        player_id: usize,
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
