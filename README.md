# Shade Nucleus

*Motor gráfico escrito en Rust — creado totalmente desde cero.*

> **Estado:** prototipo inicial (v0.1.0)  
> **Workspace:** `nucleus/` (lib) + `demo/` (ejecutable de prueba)

## ¿Qué hay dentro?

| Carpeta | Rol |
|---------|-----|
| **nucleus/** | Crate **lib** que contiene el núcleo del motor: ventana, framebuffer RGBA, primitivas básicas (`clear`, `draw_pixel`, etc.). |
| **demo/**    | Crate **bin** que usa `nucleus` para mostrar una figura en pantalla; sirve como testeo de las capacidades actuales. |

## Objetivo a corto plazo

1. Mostrar una ventana 320×240 y dibujar figuras simples **generadas por el propio motor**.  

## Cómo compilar / probar

```bash
# compila todo
cargo build

# ejecuta la demo de prueba
cargo run -p demo
```