use crate::player::Player;
use crate::framebuffer::FrameBuffer;
use crate::color::Color;
use crate::CELL_SIZE;
use nalgebra_glm as glm;

pub struct Intersect {
    pub distance:f32,
    pub impact: char,
}

pub fn cast_ray(player: &Player, map: &Vec<Vec<char>>, framebuffer: &mut FrameBuffer, a: f32, block_size: usize, draw_line: bool) -> Intersect{
     let mut d = 0.0;

    framebuffer.set_current_color(Color::new(255, 221, 221));

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.position.x + cos) as usize;
        let y = (player.position.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if map[j][i] != ' ' {
            return Intersect { 
                distance: d, 
                impact: map[j][i],
            };
        }

        if draw_line {
            framebuffer.point(x, y);
        }

        d += 0.05;
    }
}

fn is_wall(x:usize,y:usize,map:&Vec<Vec<char>>) -> bool {
    if x >= map[0].len() || y >= map.len() {
        return true;
    }
    let cell=map[y][x];
    cell == '+' || cell == '-' || cell == '|'
}
