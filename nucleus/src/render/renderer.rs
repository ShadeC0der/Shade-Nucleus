use wgpu::{
    TextureViewDescriptor, Color, CommandEncoderDescriptor,
    RenderPassDescriptor, RenderPassColorAttachment, Operations, LoadOp,
};

use winit::window::Window;
use anyhow::Result;

use super::wgpu_context::WgpuContext;
use super::surface_manager::SurfaceManager;

pub struct Renderer {
    wgpu_context: WgpuContext,
    surface_manager: SurfaceManager,
}

impl Renderer {
    /// Inicializa el renderizador y contexto gráfico.
    pub async fn new(window: &Window) -> Result<Self> {
        let wgpu_context = WgpuContext::new().await?;
        let surface_manager = SurfaceManager::new(
            window,
            &wgpu_context.instance,
            &wgpu_context.adapter,
            &wgpu_context.device,
        )?;

        Ok(Self {
            wgpu_context,
            surface_manager,
        })
    }

    /// Redimensiona el área de dibujo.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_manager
            .resize(&self.wgpu_context.device, width, height);
    }

    /// Dibuja un frame con el fondo animado.
    pub fn render(&mut self, clear_t: f32) -> Result<()> {
        let frame = self.surface_manager.get_current_texture()?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let color = Color {
            r: clear_t as f64,
            g: (1.0 - clear_t) as f64,
            b: (clear_t * 0.5) as f64,
            a: 1.0,
        };

        let mut encoder = self
            .wgpu_context
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Main Encoder"),
            });

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

            // Por ahora el pase solo limpia la pantalla: no dibuja nada más.
        }

        self.wgpu_context
            .queue
            .submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }

    /// Devuelve el nombre de la GPU utilizada.
    pub fn gpu_name(&self) -> String {
        self.wgpu_context.adapter.get_info().name.clone()
    }

    pub fn surface_width(&self) -> u32 {
        self.surface_manager.width()
    }

    pub fn surface_height(&self) -> u32 {
        self.surface_manager.height()
    }
}
