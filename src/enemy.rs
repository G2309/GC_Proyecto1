use nalgebra_glm::Vec2;
use crate::texture::Texture;

pub struct Enemy {
    pub pos: Vec2,
    pub speed: f32,
    pub is_visible: bool,
}

impl Enemy {
    pub fn new(x:f32,y:f32) -> Enemy {
        Enemy {
            pos: Vec2::new(x,y),
            speed: 3.0,
            is_visible: true,
        }
    }

    pub fn move_towards(&mut self, target: &Vec2, map: &Vec<Vec<char>>, block_size: usize) {
        let direction = (target - self.pos).normalize();
        let new_pos = self.pos + direction * self.speed;

        let i = (new_pos.x as usize) / block_size;
        let j = (new_pos.y as usize) / block_size;

        if map[j][i] == ' ' {
            self.pos = new_pos;

        }

    }
}
