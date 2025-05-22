# Shade Nucleus

*Motor gráfico escrito en Rust.*

> **Estado actual:** prototipo funcional (`v0.2.2`)  
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

- Compilar todo el workspace
```bash
cargo build
```

- Ejecutar la demo básica
```bash
cargo run -p demo
```

## 🛠️ Alternativa: usar Docker

- Construir la imagen del proyecto (compila en modo release)
```bash
docker build -t shade-nucleus .
```
- Ejecutar el contenedor (sin soporte gráfico en Windows)
```bash
docker run --rm shade-nucleus
```