//                          Framebuffer class
//                          Gustavo Cruz
//                          # 22779
use crate::color::Color;
use crate::bitmap::write_bmp_file;
use crate::texture::Texture;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub background_color: Color,
    pub current_color: Color
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        let default_color = Color::new(255, 255, 255);
        let buffer = vec![default_color; width * height];
        FrameBuffer {
            width,
            height,
            buffer,
            background_color: default_color,
            current_color: default_color
        }
    }
    // Limpia el buffer con el color de fondo
    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }
    // Dibuja un punto en las coordenadas (x, y) con el color actual
    pub fn point(&mut self, x: usize, y: usize) {
        let adjusted_y = self.height - 1 - y;
        self.buffer[self.width * y + x] = self.current_color;
        //self.buffer[self.width * y + x] = self.current_color;
    }

    // Establece el color de fondo
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
    // Obtiene el color en las coordenadas (x, y)
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.buffer[self.width * y + x]
    }
    // Establece el color actual para dibujar
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
    // Escribe el contenido del buffer en un archivo BMP
    pub fn write_to_bmp(&self, file_path: &str) -> std::io::Result<()> {
        let buffer: Vec<u32> = self.buffer.iter().map(|c| c.to_hex()).collect();
        write_bmp_file(file_path, &buffer, self.width, self.height)
    }
    // Dibuja un polígono no relleno a partir de una lista de vértices
    pub fn polygon(&mut self, vec: Vec<[usize; 2]>) {
        for i in 0..vec.len() {
            if i == vec.len() - 1 {
                self.line(vec[i][0], vec[i][1], vec[0][0], vec[0][1]);
            } else {
                self.line(vec[i][0], vec[i][1], vec[i + 1][0], vec[i + 1][1]);
            }
        }
    }

    // Dibuja un polígono relleno a partir de una lista de vértices

    pub fn draw_filled_polygon(&mut self, vec: Vec<[usize; 2]>) {
    let (min_x, max_x, min_y, max_y) = self.get_max_limits(&vec);

	    for y in min_y..=max_y {
	        let mut intersections = Vec::new();
	
	        for i in 0..vec.len() {
	            let (x0, y0, x1, y1) = if i == vec.len() - 1 {
	                (vec[i][0] as isize, vec[i][1] as isize, vec[0][0] as isize, vec[0][1] as isize)
	            } else {
	                (vec[i][0] as isize, vec[i][1] as isize, vec[i + 1][0] as isize, vec[i + 1][1] as isize)
	            };
	
	            let y = y as isize;
	
	            if (y0 <= y && y < y1) || (y1 <= y && y < y0) {
	                let x = x0 + (y - y0) * (x1 - x0) / (y1 - y0);
	                intersections.push(x as usize);
	            }
	        }
	
	        intersections.sort();
	
	        for i in (0..intersections.len()).step_by(2) {
	            if i + 1 < intersections.len() {
	                let x0 = intersections[i];
	                let x1 = intersections[i + 1];
	
	                for x in x0..=x1 {
	                    self.point(x, y as usize);
	                }
	            }
	        }
	    }
    }


    // Obtiene los límites máximos del polígono
    pub fn get_max_limits(&self, vec: &Vec<[usize; 2]>) -> (usize, usize, usize, usize) {
        let min_x = vec.iter().map(|p| p[0]).min().unwrap();
        let max_x = vec.iter().map(|p| p[0]).max().unwrap();
        let min_y = vec.iter().map(|p| p[1]).min().unwrap();
        let max_y = vec.iter().map(|p| p[1]).max().unwrap();

        (min_x, max_x, min_y, max_y)
    }

    // Dibuja una línea entre dos puntos (x0, y0) y (x1, y1)
    fn line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let (mut x0, mut y0, x1, y1) = (x0 as isize, y0 as isize, x1 as isize, y1 as isize);

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.point(x0 as usize, y0 as usize);
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw2D_texture(&mut self, texture: &Texture, x: usize, y: usize) {
        for tex_x in 0..texture.width {
            for tex_y in 0..texture.height {
                let color = texture.get_pixel_color(tex_x, tex_y);
                
                if color.r == 0 && color.g == 0 && color.b ==0 {
                    continue;
                }

                self.set_current_color(color);
                self.point(x + tex_x as usize, y + tex_y as usize);
            }
        }
    }

    pub fn draw_texture(&mut self, texture: &Texture, x: usize, y: usize) {
        for tx in 0..texture.width {
            for ty in 0..texture.height {
                let color = texture.get_pixel_color(tx, ty);
                let pixel_x = x + tx as usize;
                let pixel_y = y + ty as usize;

                // Verifica que esté dentro de los límites del framebuffer
                if pixel_x < self.width && pixel_y < self.height {
                    self.set_current_color(color);
                    self.point(pixel_x, pixel_y);
                }
            }
        }
    }

    pub fn draw_texture_scaled(
    &mut self, 
    texture: &Texture, 
    x: usize, 
    y: usize, 
    width: usize, 
    height: usize
    ) {
    for tex_x in 0..width {
        for tex_y in 0..height {
            let texture_x = (tex_x as f32 * texture.width as f32 / width as f32) as u32;
            let texture_y = (tex_y as f32 * texture.height as f32 / height as f32) as u32;
            let color = texture.get_pixel_color(texture_x, texture_y);

            if color.r == 0 && color.g == 0 && color.b == 0 {
                continue;
            }

            // Verifica que esté dentro de los límites del framebuffer
            let pixel_x = x + tex_x;
            let pixel_y = y + tex_y;
            if pixel_x < self.width && pixel_y < self.height {
                self.set_current_color(color);
                self.point(pixel_x, pixel_y);
            }
        }
    }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        for row in y..(y + height).min(self.height) {
            for col in x..(x + width).min(self.width) {
                self.set_current_color(color);
                self.point(col, row);
            }
        }
    }
    
    pub fn draw_rect_outline(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        for col in x..(x + width).min(self.width) {
            if y < self.height {  // línea superior
                self.set_current_color(color);
                self.point(col, y);
            }
            if y + height < self.height { 
                self.set_current_color(color);
                self.point(col, y + height - 1);
            }
        }
        for row in y..(y + height).min(self.height) {
            if x < self.width {  // línea izquierda
                self.set_current_color(color);
                self.point(x, row);
            }
            if x + width < self.width { 
                self.set_current_color(color);
                self.point(x + width - 1, row);
            }
        }
    }



}

