use crate::player::Player;
use crate::framebuffer::FrameBuffer;
use crate::color::Color;
use crate::CELL_SIZE;
use nalgebra_glm as glm;

pub fn cast_ray(player: &Player, map: &Vec<Vec<char>>, framebuffer: &mut FrameBuffer) {
    let num_rays = 5;
    let ray_color = Color::new(245,0,0);
    let step_size = 0.05;
    let max_distance = 3.0;
    
    for i in 0..num_rays {
        let current_ray = i as f32/num_rays as f32;
        let ray_angle = player.view_angle - (player.fov / 2.0) + (player.fov * current_ray);
	    let mut ray_pos = player.position;
	    let ray_dir = glm::vec2(ray_angle.cos(), ray_angle.sin());
        //let ray_dir = player.direction;
	
	    for _ in 0..(max_distance / step_size) as usize {
	        let map_x = ray_pos.x as usize;
	        let map_y = ray_pos.y as usize;
	
	        if is_wall(map_x, map_y, map) {
	            break;
	        }
	
	        let pixel_x = (ray_pos.x * CELL_SIZE as f32) as usize;
	        let pixel_y = framebuffer.height - ((ray_pos.y * CELL_SIZE as f32) as usize);
	
	        framebuffer.set_current_color(ray_color);
	        framebuffer.point(pixel_x, pixel_y);
	
	        ray_pos += ray_dir * step_size;
	    }
    }
}

fn is_wall(x:usize,y:usize,map:&Vec<Vec<char>>) -> bool {
    if x >= map[0].len() || y >= map.len() {
        return true;
    }
    let cell=map[y][x];
    cell == '+' || cell == '-' || cell == '|'
}
