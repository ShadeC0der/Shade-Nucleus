# Shade Nucleus

*Motor gráfico 2D escrito en Rust — creado totalmente desde cero.*

> **Estado actual:** prototipo funcional (`v0.2.0`)  
> **Estructura:** `nucleus/` (núcleo del motor) + `demo/` (ejecutable de prueba)

---

## 🚧 ¿Qué incluye esta versión?

| Carpeta      | Rol                                                                 |
|--------------|----------------------------------------------------------------------|
| **nucleus/** | Crate **lib** con el núcleo gráfico: configuración de GPU, ventana, surface, renderer y motor base (`Engine`). |
| **demo/**    | Crate **bin** de prueba. Muestra una ventana con un color animado dinámicamente. Sirve como punto de partida para futuras escenas. |

---

## 🎯 Objetivo actual

- Establecer una arquitectura modular y escalable.
- Separar la lógica (`Engine`) del renderizado (`Renderer`).
- Preparar el entorno para futuras escenas, perfiles gráficos y lógica avanzada.

---

## 🚀 Cómo compilar y ejecutar

En la ruta `/demo` ejecutar los siguientes comandos:

```bash
# Compilar todo el workspace
cargo build
```
```bash
# Ejecutar la demo básica
cargo run -p demo
```