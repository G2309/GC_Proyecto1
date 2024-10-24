use crate::texture::Texture;

pub struct EnemiesData {
    pub enemies: Vec<EnemyBattle>
}

pub struct EnemyBattle {
    pub max_hp: i32,
    pub hp: i32,
    pub max_mp: i32,
    pub mp: i32,
    pub spells: Vec<String>,
    pub enemy_texture: Texture
}

impl EnemiesData {
    pub fn new() -> Self {
        EnemiesData {
            enemies: vec![]
        }
    }

    pub fn add_enemy(&mut self, max_hp: i32, hp: i32, max_mp: i32, mp: i32, spells: Vec<String>, enemy_texture: Texture) {
        self.enemies.push(EnemyBattle {
            max_hp,
            hp,
            max_mp,
            mp,
            spells,
            enemy_texture
        });
    }
}
