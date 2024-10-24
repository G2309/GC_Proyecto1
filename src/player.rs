use nalgebra_glm::{Vec2};
use minifb::{Window, Key};
use crate::texture::Texture;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32
}

pub fn process_event(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize) {
    const move_speed: f32 = 6.5;
    const rotation_speed: f32 = 0.1;

    if window.is_key_down(Key::Left) {
        player.a -= rotation_speed;
    }
    if window.is_key_down(Key::Right) {
        player.a += rotation_speed;
    }

    let mut next_x;
    let mut next_y;

    if window.is_key_down(Key::Up) {
        next_x = player.pos.x + move_speed * player.a.cos();
        next_y = player.pos.y + move_speed * player.a.sin();
        if !is_wall(maze, next_x, next_y, block_size) {
            player.pos.x = next_x;
            player.pos.y = next_y;
        }
    }

    if window.is_key_down(Key::Down) {
        next_x = player.pos.x - move_speed * player.a.cos();
        next_y = player.pos.y - move_speed * player.a.sin();
        if !is_wall(maze, next_x, next_y, block_size) {
            player.pos.x = next_x;
            player.pos.y = next_y;
        }
    }
}

fn is_wall(map: &Vec<Vec<char>>, x: f32, y: f32, block_size: usize) -> bool {
    let j = (y / block_size as f32) as usize;
    let i = (x / block_size as f32) as usize;

    if j >= map.len() || i >= map[j].len() {
        return false;
    }

    map[j][i] != ' '
}
