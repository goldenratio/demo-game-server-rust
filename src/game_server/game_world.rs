use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PeerPlayerInfo {
    pub player_id: usize,
    pub x: f32,
    pub y: f32
}

#[derive(Debug)]
pub struct GameWorld {
    max_players_count: u8,
    current_players_count: u8,
    player_map: HashMap<usize, PeerPlayerInfo>,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self {
            max_players_count: 2,
            current_players_count: 0,
            player_map: HashMap::new()
        }
    }
}

impl GameWorld {
    pub fn add_player(&mut self, player_id: usize) {
        let peer_data = PeerPlayerInfo {
            player_id,
            x: 0.0,
            y: 0.0
        };

        if self.current_players_count >= self.max_players_count {
            // room is full
            return;
        }

        if let Some(_) = self.player_map.insert(player_id, peer_data) {
            self.current_players_count += 1;
        };
    }

    pub fn remove_player(&mut self, player_id: usize) {
        if let Some(_) = self.player_map.remove(&player_id) {
            self.current_players_count -= 1;
            // remove this game world is player count is 0
        }
    }

    pub fn update_player_position(&mut self, player_id: usize, x: f32, y: f32) {
        self.player_map.entry(player_id).and_modify(|data| {
            data.x = x;
            data.y = y;
        });
    }

    pub fn get_world_update(&self, skip_id: usize) -> Vec<PeerPlayerInfo> {
        self.player_map
            .values()
            .filter(|&x| x.player_id != skip_id)
            .cloned()
            .collect::<Vec<PeerPlayerInfo>>()
    }
}
