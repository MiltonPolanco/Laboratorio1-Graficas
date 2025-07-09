use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;

pub fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[Vertex]) {
    if vertices.len() < 3 {
        return;
    }

    // Obtener bounding box
    let mut min_x = vertices[0].x;
    let mut max_x = vertices[0].x;
    let mut min_y = vertices[0].y;
    let mut max_y = vertices[0].y;
    
    for vertex in vertices {
        min_x = min_x.min(vertex.x);
        max_x = max_x.max(vertex.x);
        min_y = min_y.min(vertex.y);
        max_y = max_y.max(vertex.y);
    }

    // Scanline por cada fila
    for y in min_y..=max_y {
        let mut intersections = Vec::new();
        
        // Buscar intersecciones con los bordes
        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            let v1 = vertices[i];
            let v2 = vertices[j];
            
            // Evitar bordes horizontales
            if v1.y == v2.y {
                continue;
            }
            
            // Ver si la línea cruza este borde
            if (v1.y <= y && y < v2.y) || (v2.y <= y && y < v1.y) {
                let x = if v2.y == v1.y {
                    v1.x.min(v2.x)
                } else {
                    let t = (y - v1.y) as f64 / (v2.y - v1.y) as f64;
                    (v1.x as f64 + t * (v2.x - v1.x) as f64).round() as i32
                };
                intersections.push(x);
            }
        }
        
        intersections.sort_unstable();
        
        // Rellenar entre pares
        let mut i = 0;
        while i < intersections.len() {
            if i + 1 < intersections.len() {
                let x1 = intersections[i];
                let x2 = intersections[i + 1];
                
                for x in x1..=x2 {
                    if x >= 0 && y >= 0 {
                        framebuffer.set_pixel(x as u32, y as u32);
                    }
                }
                i += 2;
            } else {
                break;
            }
        }
    }
}