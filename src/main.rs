mod framebuffer;
mod line;
mod vertex;
mod fill;

use crate::framebuffer::Framebuffer;
use line::line;
use vertex::Vertex;
use fill::fill_polygon;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, raylib::prelude::Color::new(50, 50, 100, 255));
    framebuffer.clear();

    // Solo polígono 3: Triángulo
    let poly3 = [
        Vertex { x: 377, y: 249 }, Vertex { x: 411, y: 197 }, Vertex { x: 436, y: 249 },
    ];

    // Rellenar triángulo en rojo
    framebuffer.set_current_color(raylib::prelude::Color::RED);
    fill_polygon(&mut framebuffer, &poly3);
    
    // Borde blanco
    framebuffer.set_current_color(raylib::prelude::Color::WHITE);
    for j in 0..poly3.len() {
        let start = poly3[j];
        let end = poly3[(j + 1) % poly3.len()];
        line(&mut framebuffer, start, end);
    }
    
    framebuffer.render_to_file("out.bmp");
}
