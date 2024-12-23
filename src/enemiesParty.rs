use crate::texture::Texture;

pub struct EnemiesData {
    pub enemies: Vec<EnemyBattle>
}

pub struct EnemyBattle {
    pub name: String,
    pub max_hp: i32,
    pub hp: i32,
    pub max_mp: i32,
    pub mp: i32,
    pub spells: Vec<String>,
    pub weakness: Vec<String>,
    pub enemy_texture: Texture
}

impl EnemiesData {
    pub fn new() -> Self {
        EnemiesData {
            enemies: vec![]
        }
    }

    pub fn add_enemy(&mut self, name: String, max_hp: i32, hp: i32, max_mp: i32, mp: i32, spells: Vec<String>, weakness: Vec<String>, enemy_texture: Texture) {
        self.enemies.push(EnemyBattle {
            name,
            max_hp,
            hp,
            max_mp,
            mp,
            spells,
            weakness,
            enemy_texture
        });
    }
}
