mod framebuffer;
mod color;
mod bitmap;
mod map_loader;
mod player;
mod raycasting;

use core::f32::consts::PI;
use crate::framebuffer::FrameBuffer;
use crate::color::Color;
use crate::bitmap::write_bmp_file;
use crate::map_loader::{load_map, print_map};
use crate::player::{Player, process_event};
use crate::raycasting::cast_ray;
use minifb::{Window, WindowOptions, Key};
use std::time::{Duration, Instant};
use nalgebra_glm::Vec2;

const WIDTH: usize = 1000;
const HEIGHT: usize = 800;
const CELL_SIZE: usize = 50;

// Definir las dimensiones y ubicaciones de las secciones
const SECTION_A_WIDTH: usize = 750;
const SECTION_A_HEIGHT: usize = 600;

const SECTION_B_WIDTH: usize = 200;
const SECTION_B_HEIGHT: usize = 200;
const SECTION_B_X: usize = WIDTH - SECTION_B_WIDTH - 10;
const SECTION_B_Y: usize = 10;

const SECTION_C_HEIGHT: usize = 150;

fn cell_to_color(cell: char) -> Color {
    match cell {
        '+' => Color::new(0, 255, 0),
        '-' => Color::new(255, 255, 0),
        '|' => Color::new(255, 165, 0),
        'p' => Color::new(0, 0, 255),
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

fn render2d(framebuffer: &mut FrameBuffer, player: &Player, maze: &Vec<Vec<char>>, block_size: usize, offset_x: usize, offset_y: usize) {
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size + offset_x, row * block_size + offset_y, block_size, maze[row][col]);
        }
    }
    framebuffer.set_current_color(Color::new(255, 0, 0));
    framebuffer.point(player.pos.x as usize + offset_x, player.pos.y as usize + offset_y);

    let num_rays = 1;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, maze, player, a, block_size, true);
    }
}

fn render3d(framebuffer: &mut FrameBuffer, player: &Player, maze: &Vec<Vec<char>>, block_size: usize, offset_x: usize, offset_y: usize, width: usize, height: usize) {
    let num_rays = width;
    let hh = height as f32 / 2.0;

    for i in 0..width {
        // Renderizar el cielo y el suelo
        for j in 0..(height as f32 / 2.0) as usize {
            framebuffer.set_current_color(Color::new(0, 0, 0));
            framebuffer.point(i + offset_x, j + offset_y);
        }
        framebuffer.set_current_color(Color::new(135, 206, 235));
        for j in (height / 2)..height {
            framebuffer.point(i + offset_x, j + offset_y);
        }
    }

    framebuffer.set_current_color(Color::new(255, 0, 0));
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, maze, player, a, block_size, false);

        let distance_to_wall = intersect.distance.max(0.1);
        let distance_to_projection_plane = 50.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;
        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(height as f32 - 1.0) as usize;

        for y in stake_top..stake_bottom {
            let color = cell_to_color(intersect.impact);
            framebuffer.set_current_color(color);
            framebuffer.point(i + offset_x, y + offset_y);
        }
    }
}

fn render_hud(framebuffer: &mut FrameBuffer, width: usize, height: usize) {
    framebuffer.set_current_color(Color::new(50, 50, 50)); // Color gris oscuro
    for y in height..HEIGHT {
        for x in 0..width {
            framebuffer.point(x, y);
        }
    }
    // Puedes agregar más detalles en el HUD aquí, como texto, barras de vida, etc.
}

fn main() {
    let window_width = WIDTH;
    let window_height = HEIGHT;
    let block_size = CELL_SIZE;
    let map = load_map("./src/map.txt");

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
        pos: Vec2::new(100.0, 100.0),
        a: 0.0,
        fov: PI / 3.0,
    };

    let mut last_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let elapsed_time = current_time.duration_since(last_time).as_secs_f64();
        last_time = current_time;

        let fps = 1.0/elapsed_time;
        framebuffer.clear();

        process_event(&window, &mut player, &map, block_size);

        // Renderizar en las diferentes secciones
        render3d(&mut framebuffer, &player, &map, block_size, 0, 0, SECTION_A_WIDTH, SECTION_A_HEIGHT);
        render2d(&mut framebuffer, &player, &map, block_size / 4, SECTION_B_X, SECTION_B_Y);
        render_hud(&mut framebuffer, WIDTH, HEIGHT - SECTION_C_HEIGHT);

        let pixel_buffer: Vec<u32> = framebuffer.buffer.iter().map(|color| color.to_u32()).collect();
        window.update_with_buffer(&pixel_buffer, framebuffer_width, framebuffer_height)
            .unwrap();
        window.set_title(&format!("Shin Megami Copia - FPS: {:.2}",fps));
        std::thread::sleep(frame_delay);
    }
}

