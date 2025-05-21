//! renderer.rs
//! 
//! Este módulo define el struct `Renderer`, que se encarga de inicializar y controlar 
//! los recursos gráficos necesarios para mostrar algo en pantalla usando `wgpu`. 
//! 
//! Su responsabilidad principal es:
//! - Orquestar la inicialización de los componentes de renderizado (`WgpuContext`, `SurfaceManager`).
//! - Proveer funciones para redimensionar la superficie de dibujo y para ejecutar el ciclo de renderizado.
//! - Encapsular la lógica de los pases de render (por ahora, un simple pase de limpieza).
//! 
//! En resumen, `Renderer` es el componente encargado de preparar y ejecutar 
//! el proceso de dibujo en una ventana gráfica, encapsulando los detalles de bajo nivel
//! relacionados con la GPU y la API de gráficos multiplataforma.
//!
//! Este módulo es autónomo: otros componentes del motor como (`Engine`) lo usan 
//! para delegar la tarea de renderizado, sin necesidad de conocer los detalles técnicos.
//! 
//! En el futuror ender solo debe controlar el frame, resize y recibir una configuracion de gpu

// Interactua con la GPU, funciona con (Vulkan, DirectX 12, Metal, OpenGL)
use wgpu::{
    // Ejemplo para el render
    TextureViewDescriptor,
    Color,
    CommandEncoderDescriptor,
    RenderPassDescriptor,
    RenderPassColorAttachment,
    Operations,
    LoadOp
};

// Crea Ventanas (Multiplataforma)
use winit::window::Window;

// Errores Genericos
use anyhow::Result;

use super::wgpu_context::WgpuContext;
use super::surface_manager::SurfaceManager;

// Estructura que agrupa Herramientas
pub struct Renderer {
    wgpu_context: WgpuContext,
    surface_manager: SurfaceManager,
}

impl Renderer {
    /// Inicializa el renderizador (GPU + área de dibujo).
    pub async fn new(window: &Window) -> Result<Self> {
        let wgpu_context = WgpuContext::new().await?;
        let surface_manager = SurfaceManager::new(
            window,
            &wgpu_context.instance,
            &wgpu_context.adapter,
            &wgpu_context.device,
        )?;

        // Retorno del Renderer con todo listo para usar
        Ok(Self { wgpu_context, surface_manager})
    }

    /// Cambia tamaño de la superficie gráfica.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_manager.resize(&self.wgpu_context.device, width, height);
    }

    /// Dibuja un frame con un color variable.
    pub fn render(&mut self, clear_t: f32) -> Result<()> {
        // Obtener el frame actual
        let frame = self.surface_manager.get_current_texture()?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        // Calcular el color del fondo
        let color = Color {
            r: clear_t as f64,
            g: (1.0 - clear_t) as f64,
            b: (clear_t * 0.5) as f64,
            a: 1.0,
        };

        // Crear encoder de comandos
        let mut encoder = self.wgpu_context.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Main Encoder"),
        });

        // Iniciar el render pass correctamente (esto asegura las transiciones necesarias)
        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        // Enviar comandos a la GPU y mostrar el resultado
        self.wgpu_context.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

      Ok(())
    }

}