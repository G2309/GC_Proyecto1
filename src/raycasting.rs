use crate::framebuffer::FrameBuffer;
use crate::player::Player;
use crate::color::Color;


pub struct Intersect {
    pub distance: f32,
    pub impact: char
}

pub fn cast_ray(framebuffer: &mut FrameBuffer, map: &Vec<Vec<char>>, player: &Player, a: f32, block_size: usize, draw_line: bool) -> Intersect {
	let mut d = 0.0;
	let mut x;
	let mut y;
	
	framebuffer.set_current_color(Color::new(255, 0, 0));
	
	loop {
	
		let cos = a.cos();
		let sin = a.sin();
		x = (player.pos.x + d * cos) as usize;
		y = (player.pos.y + d * sin) as usize;
		
		
		let i = x / block_size;
		let j = y / block_size;
		
		if draw_line {
		    framebuffer.point(x, y);  
		}
		
		if map[j][i] != ' ' {
		    return Intersect {
		        distance: d,
		        impact: map[j][i]
		    };
		}
		
		d += 0.1; 
	}
}
