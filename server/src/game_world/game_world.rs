use game_world_wasm::{add_num};

pub struct GameWorld {}

impl GameWorld {
    pub fn new() -> Self {
        add_num(2, 2);
        Self {}
    }
}