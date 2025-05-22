use wgpu::{
    TextureViewDescriptor, Color, CommandEncoderDescriptor,
    RenderPassDescriptor, RenderPassColorAttachment, Operations, LoadOp,
};

use winit::window::Window;
use anyhow::Result;

use super::wgpu_context::WgpuContext;
use super::surface_manager::SurfaceManager;
use super::ui::UiRenderer;

pub struct Renderer {
    wgpu_context: WgpuContext,
    surface_manager: SurfaceManager,
    pub ui: UiRenderer,
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

        let ui = UiRenderer::new(window, &wgpu_context.device, surface_manager.surface_format());

        Ok(Self {
            wgpu_context,
            surface_manager,
            ui,
        })
    }

    /// Redimensiona el área de dibujo.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_manager.resize(&self.wgpu_context.device, width, height);
    }

    /// Dibuja un frame con un color variable y una interfaz `egui`.
    pub fn render(
        &mut self,
        clear_t: f32,
        window: &Window,
        fps: f32,
        gpu_name: &str,
    ) -> Result<()> {
        self.ui.begin_frame(window);

        // Panel simple de diagnóstico
        egui::Window::new("Debug Info").show(&self.ui.ctx, |ui| {
            ui.label(format!("FPS: {:.1}", fps));
            ui.label(format!("GPU: {}", gpu_name));
            ui.label(format!("Clear Color t: {:.2}", clear_t));
            ui.label(format!(
                "Resolución: {} x {}",
                self.surface_manager.width(),
                self.surface_manager.height()
            ));
            ui.label(format!(
                "Formato de superficie: {:?}",
                self.surface_manager.surface_format()
            ));
            ui.label(format!(
                "Backend: {:?}",
                self.wgpu_context.adapter.get_info().backend
            ));
        });

        let (paint_jobs, screen_descriptor) = self.ui.end_frame(
            window,
            &self.wgpu_context.device,
            &self.wgpu_context.queue,
            &mut self.wgpu_context.device.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("PreEncoder"), // temporal
            }),
        );

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
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Clear Pass + Egui Render"),
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

            self.ui.render(&mut render_pass, &paint_jobs, &screen_descriptor);
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

    /// Devuelve el ancho actual de la superficie gráfica.
    pub fn surface_width(&self) -> u32 {
        self.surface_manager.width()
    }

    /// Devuelve el alto actual de la superficie gráfica.
    pub fn surface_height(&self) -> u32 {
        self.surface_manager.height()
    }
}