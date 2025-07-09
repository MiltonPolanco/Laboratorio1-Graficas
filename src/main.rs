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

    // Solo polígono 2: Cuadrilátero
    let poly2 = [
        Vertex { x: 321, y: 335 }, Vertex { x: 288, y: 286 },
        Vertex { x: 339, y: 251 }, Vertex { x: 374, y: 302 },
    ];

    // Rellenar cuadrilátero en azul
    framebuffer.set_current_color(raylib::prelude::Color::BLUE);
    fill_polygon(&mut framebuffer, &poly2);
    
    // Borde blanco
    framebuffer.set_current_color(raylib::prelude::Color::WHITE);
    for j in 0..poly2.len() {
        let start = poly2[j];
        let end = poly2[(j + 1) % poly2.len()];
        line(&mut framebuffer, start, end);
    }
    
    framebuffer.render_to_file("out.bmp");
}
