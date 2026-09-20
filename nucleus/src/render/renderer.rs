use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, TextureViewDescriptor,
};

use anyhow::Result;
use std::sync::Arc;
use winit::window::Window;

use super::surface_manager::SurfaceManager;
use super::wgpu_context::WgpuContext;

pub struct Renderer {
    wgpu_context: WgpuContext,
    surface_manager: SurfaceManager,
}

impl Renderer {
    /// Inicializa el renderizador y contexto gráfico.
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let wgpu_context = WgpuContext::new().await?;
        let surface_manager = SurfaceManager::new(
            window.clone(),
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

        // Tres ondas desfasadas un tercio de vuelta: el color recorre el círculo
        // cromático y vuelve al punto de partida sin dar ningún salto brusco.
        let turn = clear_t as f64 * std::f64::consts::TAU;
        let wave = |phase: f64| 0.5 + 0.5 * (turn + phase).sin();

        let color = Color {
            r: wave(0.0),
            g: wave(std::f64::consts::TAU / 3.0),
            b: wave(2.0 * std::f64::consts::TAU / 3.0),
            a: 1.0,
        };

        let mut encoder =
            self.wgpu_context
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
                    depth_slice: None,
                    ops: Operations {
                        load: LoadOp::Clear(color),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            // Por ahora el pase solo limpia la pantalla: no dibuja nada más.
        }

        self.wgpu_context
            .queue
            .submit(std::iter::once(encoder.finish()));
        // Presentar ya no es cosa de la textura, sino de la cola
        self.wgpu_context.queue.present(frame);

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
