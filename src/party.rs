use crate::texture::Texture;

pub struct Party {
    pub players_data: Vec<PlayerData>
}

pub struct PlayerData {
    pub max_hp: i32,
    pub hp: i32,
    pub max_mp: i32,
    pub mp: i32,
    pub texture: Texture,
}

impl Party {
    pub fn new() -> Self {
        Party {
            players_data: vec![],
        }
    }

    pub fn add_player(&mut self, max_hp: i32, hp: i32, max_mp: i32, mp: i32, texture: Texture) {
        self.players_data.push(PlayerData {
            max_hp,
            hp,
            max_mp,
            mp,
            texture
        });
    }
}
