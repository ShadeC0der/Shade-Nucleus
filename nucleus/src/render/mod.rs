//! El módulo `render` es responsable de toda la lógica relacionada con la
//! representación gráfica en la pantalla.
//!
//! Se subdivide en los siguientes componentes principales:
//! - `wgpu_context`: Gestiona la inicialización y el acceso a los objetos
//!   fundamentales de WGPU como la instancia, el adaptador, el dispositivo y la cola.
//! - `surface_manager`: Administra la superficie de dibujo (`Surface`) asociada a la
//!   ventana y su configuración (`SurfaceConfiguration`).
//! - `renderer`: Es el orquestador principal que utiliza `WgpuContext` y
//!   `SurfaceManager` para ejecutar los pases de renderizado y presentar
//!   los frames en la pantalla. Expone la API pública para operaciones de
//!   renderizado como `Renderer::new()`, `Renderer::resize()` y `Renderer::render()`.

pub mod renderer;
pub mod wgpu_context;
pub mod surface_manager;
