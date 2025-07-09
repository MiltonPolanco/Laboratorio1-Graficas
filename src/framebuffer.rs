use raylib::prelude::*;

// Framebuffer: imagen donde vamos a dibujar
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    // Crea un framebuffer nuevo
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    // Limpia la imagen con el color de fondo
    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }

    // Dibuja un pixel en (x, y)
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    // Cambia el color de fondo y limpia
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.clear();
    }

    // Cambia el color actual
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    // Guarda la imagen en un archivo
    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
}
