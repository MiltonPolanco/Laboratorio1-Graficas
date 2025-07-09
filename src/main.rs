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

    // Solo polígono 1: Estrella
    let poly1 = [
        Vertex { x: 165, y: 380 }, Vertex { x: 185, y: 360 }, Vertex { x: 180, y: 330 },
        Vertex { x: 207, y: 345 }, Vertex { x: 233, y: 330 }, Vertex { x: 230, y: 360 },
        Vertex { x: 250, y: 380 }, Vertex { x: 220, y: 385 }, Vertex { x: 205, y: 410 },
        Vertex { x: 193, y: 383 },
    ];

    // Rellenar estrella en amarillo
    framebuffer.set_current_color(raylib::prelude::Color::YELLOW);
    fill_polygon(&mut framebuffer, &poly1);
    
    // Borde blanco
    framebuffer.set_current_color(raylib::prelude::Color::WHITE);
    for j in 0..poly1.len() {
        let start = poly1[j];
        let end = poly1[(j + 1) % poly1.len()];
        line(&mut framebuffer, start, end);
    }
    
    framebuffer.render_to_file("out.bmp");
}
