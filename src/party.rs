use crate::texture::Texture;

pub struct Party {
    pub players_data: Vec<PlayerData>
}

pub struct PlayerData {
    pub name: String,
    pub max_hp: i32,
    pub hp: i32,
    pub max_mp: i32,
    pub mp: i32,
    pub spells: Vec<String>,
    pub weakness: Vec<String>,
    pub texture: Texture,
    pub is_defending: bool,
}

impl Party {
    pub fn new() -> Self {
        Party {
            players_data: vec![],
        }
    }

    pub fn add_player(&mut self, name: String, max_hp: i32, hp: i32, max_mp: i32, mp: i32, spells: Vec<String>, weakness: Vec<String>, texture: Texture, is_defending: bool) {
        self.players_data.push(PlayerData {
            name,
            max_hp,
            hp,
            max_mp,
            mp,
            spells,
            weakness,
            texture,
            is_defending
        });
    }
}
