use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct PeerPlayerInfo {
    pub player_id: usize,
    pub x: f32,
    pub y: f32
}

#[derive(Debug)]
pub struct GameWorld {
    player_map: HashMap<usize, PeerPlayerInfo>,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self {
            player_map: HashMap::new()
        }
    }
}

impl GameWorld {
    pub fn add_player(&mut self, player_id: usize) {
        self.player_map.insert(player_id, PeerPlayerInfo {
            player_id,
            x: 0.0,
            y: 0.0
        });
    }

    pub fn remove_player(&mut self, player_id: usize) {
        self.player_map.remove(&player_id);
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
