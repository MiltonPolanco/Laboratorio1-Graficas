# Lab1-Graficas

Implementación de algoritmo de relleno de polígono.

## Descripción
Proyecto que implementa relleno de polígonos usando algoritmo scanline. Dibuja 4 polígonos con diferentes colores y uno con agujero.

## Polígonos incluidos
1. Estrella de 10 puntas - Amarillo
2. Cuadrilátero - Azul  
3. Triángulo - Rojo
4. Polígono complejo con agujero - Verde

## Uso
```bash
cargo run
```

## Salida
Se genera el archivo `out.bmp` con los polígonos renderizados.

## Archivos principales
- `src/main.rs` - Programa principal
- `src/framebuffer.rs` - Manejo del framebuffer
- `src/fill.rs` - Algoritmo de relleno
- `src/line.rs` - Dibujo de líneas
- `src/vertex.rs` - Estructura de vértices