// src/main.rs

mod framebuffer;
mod color;
mod bitmap;
mod map_loader;

use crate::framebuffer::FrameBuffer;
use crate::color::Color;
use crate::bitmap::write_bmp_file;
use crate::map_loader::load_map;

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let map = load_map("map.txt");
    //
    // IMPORTANTE: BORRAR LA IMPRESION DEL MAPA CARGADO
    //
    println!("Mapa cargado:");
    crate::map_loader::print_map(&map);

    let window_width = WIDTH;
    let window_height = HEIGHT;

    let framebuffer_width = WIDTH;
    let framebuffer_height = HEIGHT;

    let frame_delay = Duration::from_millis(75);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Shin Megami Copia",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Error al crear la ventana: {}", e);
    });

    window.limit_update_rate(Some(frame_delay));

}
