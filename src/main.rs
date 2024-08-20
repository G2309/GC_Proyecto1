mod framebuffer;
mod color;
mod bitmap;
mod map_loader;
mod player;
mod raycasting;
mod input;

use crate::framebuffer::FrameBuffer;
use crate::color::Color;
use crate::bitmap::write_bmp_file;
use crate::map_loader::load_map;
use crate::player::Player;
use crate::raycasting::cast_ray;

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const CELL_SIZE: usize = 50;
const PROJECTION_PLANE_DISTANCE: f32 = 277.0; 

// Función render para el mapa
fn render_map(framebuffer: &mut FrameBuffer, map: &Vec<Vec<char>>, player: &Player) {
    let cell_size = CELL_SIZE;
    let background_color = Color::new(255, 255, 255);
    let wall_color = Color::new(0, 0, 0);
    let path_color = Color::new(200, 200, 200);
    let player_color = Color::new(0, 0, 255);
    let goal_color = Color::new(0, 255, 0);

    framebuffer.clear();

    // Dibuja el mapa
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let color = match cell {
                '+' | '|' | '-' => wall_color,
                ' ' => path_color,
                'g' => goal_color,
                _ => background_color,
            };

            let y_revert = framebuffer.height - (y * cell_size) - cell_size;
            framebuffer.set_current_color(color);

            for dy in 0..cell_size {
                for dx in 0..cell_size {
                    framebuffer.point(x * cell_size + dx, y_revert + dy);
                }
            }
        }
    }

    // Dibuja al jugador en su posición actual
    let player_x = (player.position.x * cell_size as f32) as usize;
    let player_y = (player.position.y * cell_size as f32) as usize;

    framebuffer.set_current_color(player_color);

    let y_revert = framebuffer.height - player_y - cell_size;
    for dy in 0..cell_size {
        for dx in 0..cell_size {
            framebuffer.point(player_x + dx, y_revert + dy);
        }
    }
}


// Nueva función render_3D para dibujar en 3D con "stakes"

fn main() {
    let map = load_map("map.txt");

    let mut player_pos = (0.0, 0.0);
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'p' {
                player_pos = (x as f32, y as f32);
            }
        }
    }

    let mut player = Player::new(player_pos.0, player_pos.1, PI / 3.0, PI / 3.0);

    let window_width = WIDTH;
    let window_height = HEIGHT;

    let framebuffer_width = WIDTH;
    let framebuffer_height = HEIGHT;

    let frame_delay = Duration::from_millis(50);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Shin Megami Copia | Esc para Salir",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Error al crear la ventana: {}", e);
    });

    window.limit_update_rate(Some(frame_delay));

    let mut mode = "2D";

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        if mode == "2D" {
            render_map(&mut framebuffer, &map, &player);
        } else {
            break;
        }

        input::process_events(&mut player, &window);

        let buffer: Vec<u32> = framebuffer
            .buffer
            .iter()
            .map(|c| ((c.r as u32) << 16) | ((c.g as u32) << 8) | (c.b as u32))
            .collect();

        window
            .update_with_buffer(&buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    }
}

