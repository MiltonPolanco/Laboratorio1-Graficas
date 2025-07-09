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

    // Coordenadas de los polígonos
    let poly1 = [
        Vertex { x: 165, y: 380 }, Vertex { x: 185, y: 360 }, Vertex { x: 180, y: 330 },
        Vertex { x: 207, y: 345 }, Vertex { x: 233, y: 330 }, Vertex { x: 230, y: 360 },
        Vertex { x: 250, y: 380 }, Vertex { x: 220, y: 385 }, Vertex { x: 205, y: 410 },
        Vertex { x: 193, y: 383 },
    ];

    let poly2 = [
        Vertex { x: 321, y: 335 }, Vertex { x: 288, y: 286 },
        Vertex { x: 339, y: 251 }, Vertex { x: 374, y: 302 },
    ];

    let poly3 = [
        Vertex { x: 377, y: 249 }, Vertex { x: 411, y: 197 }, Vertex { x: 436, y: 249 },
    ];

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

    let polys = [&poly1[..], &poly2[..], &poly3[..], &poly4[..]];
    let colors = [
        raylib::prelude::Color::YELLOW,
        raylib::prelude::Color::BLUE,
        raylib::prelude::Color::RED,
        raylib::prelude::Color::GREEN,
    ];

    // Dibujar polígonos
    for (i, poly) in polys.iter().enumerate() {
        framebuffer.set_current_color(colors[i]);
        fill_polygon(&mut framebuffer, poly);
        
        // Crear agujero en el polígono 4
        if i == 3 {
            framebuffer.set_current_color(raylib::prelude::Color::new(50, 50, 100, 255));
            fill_polygon(&mut framebuffer, &poly5);
        }
        
        // Dibujar bordes
        framebuffer.set_current_color(raylib::prelude::Color::WHITE);
        for j in 0..poly.len() {
            let start = poly[j];
            let end = poly[(j + 1) % poly.len()];
            line(&mut framebuffer, start, end);
        }
    }
    
    // Borde del agujero
    for j in 0..poly5.len() {
        let start = poly5[j];
        let end = poly5[(j + 1) % poly5.len()];
        line(&mut framebuffer, start, end);
    }
    
    framebuffer.render_to_file("out.bmp");
}
