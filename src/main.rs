// main.rs

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

    // Polígono 4: Forma compleja
    let poly4 = [
        Vertex { x: 413, y: 177 }, Vertex { x: 448, y: 159 }, Vertex { x: 502, y: 88 },
        Vertex { x: 553, y: 53 }, Vertex { x: 535, y: 36 }, Vertex { x: 676, y: 37 },
        Vertex { x: 660, y: 52 }, Vertex { x: 750, y: 145 }, Vertex { x: 761, y: 179 },
        Vertex { x: 672, y: 192 }, Vertex { x: 659, y: 214 }, Vertex { x: 615, y: 214 },
        Vertex { x: 632, y: 230 }, Vertex { x: 580, y: 230 }, Vertex { x: 597, y: 215 },
        Vertex { x: 552, y: 214 }, Vertex { x: 517, y: 144 }, Vertex { x: 466, y: 180 },
    ];

    // Agujero
    let poly5 = [
        Vertex { x: 682, y: 175 }, Vertex { x: 708, y: 120 },
        Vertex { x: 735, y: 148 }, Vertex { x: 739, y: 170 },
    ];

    // Rellenar polígono en verde
    framebuffer.set_current_color(raylib::prelude::Color::GREEN);
    fill_polygon(&mut framebuffer, &poly4);
    
    // Crear agujero
    framebuffer.set_current_color(raylib::prelude::Color::new(50, 50, 100, 255));
    fill_polygon(&mut framebuffer, &poly5);
    
    // Bordes blancos
    framebuffer.set_current_color(raylib::prelude::Color::WHITE);
    for j in 0..poly4.len() {
        let start = poly4[j];
        let end = poly4[(j + 1) % poly4.len()];
        line(&mut framebuffer, start, end);
    }
    
    // Borde del agujero
    for j in 0..poly5.len() {
        let start = poly5[j];
        let end = poly5[(j + 1) % poly5.len()];
        line(&mut framebuffer, start, end);
    }
    
    framebuffer.render_to_file("out.bmp");
}
