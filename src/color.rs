//                          Color class
//                          Gustavo Cruz
//                          #22779
#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    // Constructor que recibe valores RGB
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    // Constructor que recibe un valor hexadecimal
    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Self { r, g, b }
    }

    // Método que retorna el valor hexadecimal del color
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    // Método que permite sumar dos colores
    pub fn add(&self, other: &Color) -> Self {
        Self {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }

    // Método que permite multiplicar un color por una constante
    pub fn multiply(&self, factor: f32) -> Self {
        Self {
            r: ((self.r as f32 * factor).clamp(0.0, 255.0)) as u8,
            g: ((self.g as f32 * factor).clamp(0.0, 255.0)) as u8,
            b: ((self.b as f32 * factor).clamp(0.0, 255.0)) as u8,
        }
    }

    // Método para imprimir el color
    pub fn print(&self) {
        println!("Color (r: {}, g: {}, b: {})", self.r, self.g, self.b);
    }
}

