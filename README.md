# Shade Nucleus

*Motor gráfico escrito en Rust, sobre `winit` y `wgpu`.*

> **Estado:** `v0.3.0` — abre una ventana con un fondo animado.
> Todavía no dibuja geometría propia: no hay shaders ni pipelines.

---

## Estructura

| Carpeta | Rol |
|---|---|
| **nucleus/** | Crate **lib**: el núcleo gráfico. Contexto de GPU, superficie, renderer y el motor (`Engine`). |
| **demo/** | Crate **bin**: ejecutable de prueba. Abre una ventana con un fondo que cambia de color. |

Dentro de `nucleus/src`:

| Módulo | Qué hace |
|---|---|
| `core/engine.rs` | El motor: mide el tiempo, cuenta fotogramas y pide dibujar. |
| `render/wgpu_context.rs` | Instancia, adaptador, dispositivo y cola de wgpu. |
| `render/surface_manager.rs` | La superficie de la ventana y su configuración. |
| `render/renderer.rs` | Arma y envía cada fotograma. |
| `utils/messages.rs` | Mensajes de error. |

---

## Cómo compilar y ejecutar

```bash
cargo build          # compila el workspace
cargo run -p demo    # ejecuta la demo
```

Para ver los mensajes de registro:

```bash
RUST_LOG=info cargo run -p demo
```

### Compilación de publicación

```bash
cargo build --release
```

El perfil de release está afinado para que el binario pese lo menos posible:
`strip`, LTO, una sola unidad de generación de código y aborto en caso de
`panic`. Resultado actual: **5,05 MB**, frente a los 12,30 MB del perfil por
defecto.

---

## Requisitos

| | |
|---|---|
| Rust | 1.98 o superior (edición 2024) |
| Linux | Vulkan. Funciona sobre Wayland y X11 |
| Windows | DirectX 12 o Vulkan |
| macOS | Metal |

Cada sistema compila únicamente su backend gráfico, declarado por objetivo en
`nucleus/Cargo.toml`.

> En Wayland las decoraciones de ventana las dibuja el compositor: el motor no
> incluye decoraciones del lado del cliente.

---

## Estado y próximos pasos

Lo que ya funciona:

- Ventana, bucle de eventos y redimensionado.
- Contexto de GPU completo y superficie configurada.
- Fondo animado por tiempo transcurrido, sincronizado con la pantalla.
- Contador de fotogramas por segundo.

Lo siguiente:

1. Primer shader en WGSL y su pipeline: dibujar un triángulo.
2. Dibujar muchos rectángulos con instanciación.
3. Exponer el estado del teclado.
4. Texturas y atlas de fuente.
5. Una rejilla de glifos.

---

## Alternativa: compilar con Docker

Compila el ejecutable en release dentro de un entorno limpio. **No sirve para
ejecutar el juego**: un contenedor no tiene acceso a la pantalla ni a la GPU sin
configuración adicional.

```bash
docker build -t shade-nucleus .
```
