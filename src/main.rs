mod framebuffer;
mod color;
mod bitmap;
mod map_loader;
mod player;
mod raycasting;
mod enemy;
mod actions;
mod texture;

use core::f32::consts::PI;
use crate::framebuffer::FrameBuffer;
use crate::color::Color;
use crate::map_loader::load_map;
use crate::player::{Player, process_event};
use crate::raycasting::cast_ray;
use minifb::{Window, WindowOptions, Key};
use std::time::{Duration, Instant};
use crate::enemy::Enemy;
use crate::actions::Actions;
use texture::Texture;
use std::collections::HashMap;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use nalgebra_glm::Vec2;

const WIDTH: usize = 1000;
const HEIGHT: usize = 800;
const CELL_SIZE: usize = 50;

enum GameState {
    Title,
    Menu,
    Playing,
}

fn cell_to_color(cell: char) -> Color {
    match cell {
        '+' => Color::new(0, 255, 0),
        '-' => Color::new(255, 255, 0),
        '|' => Color::new(255, 165, 0),
        'g' => Color::new(165, 165, 135),
        ' ' => Color::new(200, 200, 200),
        _ => Color::new(255, 255, 255),
    }
}

fn draw_cell(framebuffer: &mut FrameBuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            if cell != ' ' {
                let color = cell_to_color(cell);
                framebuffer.set_current_color(color);
                framebuffer.point(x, y);
            }
        }
    }
}

fn render2d(framebuffer: &mut FrameBuffer, player: &Player, maze: &Vec<Vec<char>>, block_size: usize, xo: usize, yo: usize, scale_factor: f32, enemies: &Vec<Enemy>, player_texture: &Texture, enemy_texture: &Texture) {
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let scaled_block_size = (block_size as f32 * scale_factor) as usize;
            let scaled_x = (xo as f32 + (col as f32 * scaled_block_size as f32)) as usize;
            let scaled_y =  (yo as f32 + (row as f32 * scaled_block_size as f32)) as usize;
            draw_cell(framebuffer, scaled_x, scaled_y, scaled_block_size, maze[row][col]);
        }
    }

    let player_x = (xo as f32 + (player.pos.x as f32 * scale_factor)) as usize;
    let player_y = (yo as f32 + (player.pos.y as f32 * scale_factor)) as usize;
    framebuffer.draw2D_texture(player_texture, player_x, player_y);

    for enemy in enemies {
        let enemy_x = (xo as f32 + (enemy.pos.x as f32 * scale_factor)) as usize;
        let enemy_y = (yo as f32 + (enemy.pos.y as f32 * scale_factor)) as usize;
        framebuffer.draw2D_texture(enemy_texture, enemy_x, enemy_y);
    }
}

fn render3d(framebuffer: &mut FrameBuffer, player: &Player, maze: &Vec<Vec<char>>, block_size: usize, textures:&Vec<Texture>, wall_texture: &HashMap<char,usize>) {
    let num_rays = framebuffer.width;

    for i in 0..num_rays {
        for j in 0..(framebuffer.height as f32 / 2.0) as usize {
            framebuffer.set_current_color(Color::new(0, 0, 0)); 
            framebuffer.point(i, j);
        }

        framebuffer.set_current_color(Color::new(135, 206, 235)); 
        for j in (framebuffer.height / 2)..framebuffer.height {
            framebuffer.point(i, j);
        }
    }

    let hh = framebuffer.height as f32 / 2.0;
    framebuffer.set_current_color(Color::new(255, 0, 0));

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, maze, player, a, block_size, false);

        let distance_to_wall = intersect.distance.max(0.1);
        let distance_to_projection_plane = 50.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;
        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32 - 1.0) as usize;
        let wall_char = intersect.impact;
        if let Some(&texture_index) = wall_texture.get(&wall_char) {
            let wall_tex = &textures[texture_index];
	        
	        let texture_x = ((intersect.distance % 1.0) * wall_tex.width as f32) as u32; 	
	        for y in stake_top..stake_bottom {
	            let texture_y = (((y as f32 - stake_top as f32) / stake_height as f32) * wall_tex.height as f32) as u32;
	            let color = wall_tex.get_pixel_color(texture_x, texture_y);
	            framebuffer.set_current_color(color);
	            framebuffer.point(i, y);
            }
        }
    }
}


fn move_enemies(enemies: &mut Vec<Enemy>, player: &Player, map: &Vec<Vec<char>>, block_size: usize, framebuffer: &mut FrameBuffer, scale_factor: f32, xo: usize, yo: usize) {
    for enemy in enemies.iter_mut() {
        let distance = (enemy.pos - player.pos).magnitude();
        if distance <= block_size as f32 * 2.0 {
            enemy.move_towards(&player.pos, map, block_size);
        }
        framebuffer.set_current_color(Color::new(255,255,255));
        let enemy_x = (xo as f32 + (enemy.pos.x as f32 * scale_factor)) as usize;
        let enemy_y = (yo as f32 + (enemy.pos.y as f32 * scale_factor)) as usize;
        framebuffer.point(enemy_x, enemy_y);
    }
}

fn move_enemies_3d(enemies: &mut Vec<Enemy>, player: &Player, map: &Vec<Vec<char>>, block_size: usize) {
    for enemy in enemies.iter_mut() {
        let distance = (enemy.pos - player.pos).magnitude();
        if distance <= block_size as f32 * 2.0 {
            enemy.move_towards(&player.pos, map, block_size);
        }
    }
}

fn apply_billboarding(enemy_pos: &Vec2, player_pos: &Vec2) -> f32 {
    let delta_x = enemy_pos.x as f32- player_pos.x as f32;
    let delta_y = enemy_pos.y as f32 - player_pos.y as f32;
    let angle = delta_y.atan2(delta_x);

    if angle > PI{
        angle - 2.0 * PI
    } else if angle < -PI {
        angle + 2.0 * PI
    } else {
        angle
    }
}

fn render_enemies_3d(
    framebuffer: &mut FrameBuffer, 
    enemies: &Vec<Enemy>, 
    player: &Player, 
    enemy_texture: &Texture, 
    block_size: usize, 
    screen_distance: f32,
    map: &Vec<Vec<char>>,
) {
    for enemy in enemies {
        let distance = (enemy.pos - player.pos).magnitude();

        let ray_result = cast_ray(framebuffer, map, player, (enemy.pos - player.pos).angle(&Vec2::new(1.0,0.0)), block_size, false);

        if ray_result.distance < distance {
            continue;
        }

        let scale_factor = block_size as f32 / distance; 
        let enemy_screen_x = framebuffer.width as f32 / 2.0 + (enemy.pos.x - player.pos.x) * screen_distance / distance;
        
        let angle_to_player = apply_billboarding(&enemy.pos, &player.pos);
        let sprite_width = enemy_texture.width as f32 * scale_factor;
        let sprite_height = enemy_texture.height as f32 * scale_factor;

        framebuffer.draw_texture_rotated(enemy_texture, enemy_screen_x as usize, angle_to_player, sprite_width as usize, sprite_height as usize);
    }
}


fn play_audio(file_path: &str, sink: &Sink) {
    let file = File::open(file_path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
}

fn stop_audio(sink: &Sink) {
    sink.stop(); 
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let menu_sink = Sink::try_new(&stream_handle).unwrap();
    let game_sink = Sink::try_new(&stream_handle).unwrap();

    let window_width = WIDTH;
    let window_height = HEIGHT;
    let block_size = CELL_SIZE;
    
    let map_data = load_map("./src/map.txt");
    let mut map = map_data.map;

    let textures = vec![
        Texture::new("src/textures/WallB1.png"),
        Texture::new("src/textures/WallY.png"),
        Texture::new("src/textures/WallG.png"),
    ];

    let title_texture = Texture::new("src/textures/title.jpg");
    let menu_texture = Texture::new("src/textures/menu.jpg");

    let player_texture = Texture::new("src/textures/player-2D.png");
    let enemy_texture = Texture::new("src/textures/enemy-2D.png");
    let enemy_texture3d = Texture::new("src/textures/enemy.png");

    let wall_texture: HashMap<char, usize> = HashMap::from([
        ('+', 0),
        ('|', 0),
        ('-', 0),
        ('d', 1),
    ]);

    let framebuffer_width = WIDTH;
    let framebuffer_height = HEIGHT;

    let frame_delay = Duration::from_millis(30);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_current_color(Color::new(50, 50, 100));

    let mut window = Window::new(
        "Shin Megami Copia - 2D/3D View",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Unable to create window: {}", e);
    });

    let mut player = Player {
        pos: map_data.player_pos * block_size as f32,
        a: 0.0,
        fov: PI / 3.0,
    };

    let mut enemies = Vec::new();
    for enemy_pos in map_data.enemies_pos {
        let enemy = Enemy::new(enemy_pos.x * block_size as f32, enemy_pos.y * block_size as f32);
        enemies.push(enemy);
    }

    let mut last_time = Instant::now();

    let scale_factor = 0.38;
    let xo = WIDTH - WIDTH / 4;
    let yo = 0;

    let mut current_state = GameState::Title;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let elapsed_time = current_time.duration_since(last_time).as_secs_f64();
        last_time = current_time;

        let fps = 1.0 / elapsed_time;
        framebuffer.clear();

        match current_state {
            GameState::Title => {
                framebuffer.draw_texture(&title_texture, 0, 0); 
                play_audio("src/music/game.mp3",&menu_sink);
                if window.is_key_down(Key::C) {
                    current_state = GameState::Menu; 
                }
            },
            GameState::Menu => {
                framebuffer.draw_texture(&menu_texture, 0, 0);
                if window.is_key_down(Key::B) {
                    current_state = GameState::Playing;
                    stop_audio(&menu_sink);
                    play_audio("src/music/game1.mp3", &game_sink);
                } else if window.is_key_down(Key::R) {
                    current_state = GameState::Title; 
                }
            },
            GameState::Playing => {
                if window.is_key_down(Key::E) {
                    Actions::check_doors(&player,&mut map);
                }
                process_event(&window, &mut player, &map, block_size);
                move_enemies(&mut enemies, &player, &map, block_size, &mut framebuffer, scale_factor, xo, yo);
                move_enemies_3d(&mut enemies, &player, &map, block_size);
                render3d(&mut framebuffer, &player, &map, block_size, &textures, &wall_texture);
                render_enemies_3d(&mut framebuffer, &enemies, &player, &enemy_texture3d, block_size, 50.0,&map);
                render2d(&mut framebuffer, &player, &map, block_size, xo, yo, scale_factor, &enemies, &player_texture,&enemy_texture);
            },
        }

        let pixel_buffer: Vec<u32> = framebuffer.buffer.iter().map(|color| color.to_u32()).collect();
        window
            .update_with_buffer(&pixel_buffer, framebuffer_width, framebuffer_height)
            .unwrap();
        window.set_title(&format!("Shin Megami Copia - FPS: {:.2}", fps));
        std::thread::sleep(frame_delay);
    }

}

