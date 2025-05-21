//! renderer.rs
//! 
//! Este módulo define el struct `Renderer`, que se encarga de inicializar y controlar 
//! los recursos gráficos necesarios para mostrar algo en pantalla usando `wgpu`. 

use wgpu::{
    TextureViewDescriptor, Color, CommandEncoderDescriptor,
    RenderPassDescriptor, RenderPassColorAttachment, Operations, LoadOp,
};

use winit::window::Window;
use anyhow::Result;

use super::wgpu_context::WgpuContext;
use super::surface_manager::SurfaceManager;

// egui
use egui::{Context as EguiContext, Visuals};
use egui_wgpu::renderer::{Renderer as EguiWgpuRenderer, ScreenDescriptor};
use egui_winit::State as EguiWinitState;

pub struct Renderer {
    wgpu_context: WgpuContext,
    surface_manager: SurfaceManager,
    pub egui_ctx: EguiContext,
    pub egui_state: EguiWinitState,
    egui_wgpu_renderer: EguiWgpuRenderer,
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

        let egui_ctx = EguiContext::default();
        egui_ctx.set_visuals(Visuals::dark());

        let scale_factor = window.scale_factor();
        let max_texture_side = wgpu_context.device.limits().max_texture_dimension_2d as usize;

        let mut egui_state = EguiWinitState::new(window);
        egui_state.set_max_texture_side(max_texture_side);
        egui_state.set_pixels_per_point(scale_factor as f32);

        let egui_wgpu_renderer = EguiWgpuRenderer::new(
            &wgpu_context.device,
            surface_manager.surface_format(),
            None,
            1,
        );

        Ok(Self {
            wgpu_context,
            surface_manager,
            egui_ctx,
            egui_state,
            egui_wgpu_renderer,
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
        let raw_input = self.egui_state.take_egui_input(window);
        self.egui_ctx.begin_frame(raw_input);

        // Panel simple de diagnóstico
        egui::Window::new("Debug Info").show(&self.egui_ctx, |ui| {
            ui.label(format!("FPS: {:.1}", fps));
            ui.label(format!("GPU: {}", gpu_name));
            ui.label(format!("Clear Color t: {:.2}", clear_t));
        });

        let full_output = self.egui_ctx.end_frame();
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, full_output.platform_output);

        let paint_jobs = self.egui_ctx.tessellate(full_output.shapes);

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

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [
                self.surface_manager.width(),
                self.surface_manager.height(),
            ],
            pixels_per_point: window.scale_factor() as f32,
        };

        for (id, delta) in &full_output.textures_delta.set {
            self.egui_wgpu_renderer.update_texture(
                &self.wgpu_context.device,
                &self.wgpu_context.queue,
                *id,
                delta,
            );
        }
        for id in &full_output.textures_delta.free {
            self.egui_wgpu_renderer.free_texture(id);
        }

        self.egui_wgpu_renderer.update_buffers(
            &self.wgpu_context.device,
            &self.wgpu_context.queue,
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );

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

            self.egui_wgpu_renderer
                .render(&mut render_pass, &paint_jobs, &screen_descriptor);
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
