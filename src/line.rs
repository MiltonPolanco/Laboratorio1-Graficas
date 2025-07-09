// line.rs
use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;

/// Dibuja una línea entre dos vértices usando el algoritmo de Bresenham.
/// Los puntos se dibujan en el framebuffer usando el color actual.
pub fn line(
    framebuffer: &mut Framebuffer,
    start: Vertex,
    end: Vertex,
) {
    // Algoritmo de Bresenham para líneas
    let mut x0 = start.x;
    let mut y0 = start.y;
    let x1 = end.x;
    let y1 = end.y;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        framebuffer.set_pixel(x0 as u32, y0 as u32);
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}