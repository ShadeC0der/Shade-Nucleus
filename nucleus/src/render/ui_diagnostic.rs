use egui::{Context as EguiContext, Visuals};
use egui_wgpu::renderer::{Renderer as EguiWgpuRenderer, ScreenDescriptor};
use egui_winit::State as EguiWinitState;
use winit::event::WindowEvent;
use winit::window::Window;
use wgpu::{Device, Queue};

///! El módulo `ui` es responsable de la interfaz de usuario utilizando `egui` y `wgpu`.
pub struct UiRenderer {
    pub ctx: EguiContext,
    pub state: EguiWinitState,
    pub renderer: EguiWgpuRenderer,
}

///! La estructura `UiRenderer` encapsula el contexto de `egui`, el estado de `egui_winit`
impl UiRenderer {
    ///! Proporciona métodos para iniciar y finalizar
    pub fn new(window: &Window, device: &Device, surface_format: wgpu::TextureFormat) -> Self {
        let ctx = EguiContext::default();
        ctx.set_visuals(Visuals::dark());

        let scale_factor = window.scale_factor();
        let mut state = EguiWinitState::new(window);
        state.set_max_texture_side(device.limits().max_texture_dimension_2d as usize);
        state.set_pixels_per_point(scale_factor as f32);

        let renderer = EguiWgpuRenderer::new(device, surface_format, None, 1);

        Self { ctx, state, renderer }
    }

    ///! Inicia un nuevo frame de `egui` y procesa la entrada del usuario.
    pub fn begin_frame(&mut self, window: &Window) {
        let raw_input = self.state.take_egui_input(window);
        self.ctx.begin_frame(raw_input);
    }

    ///! Finaliza el frame de `egui`, procesa las primitivas y actualiza los buffers.
    pub fn end_frame(
        &mut self,
        window: &Window,
        device: &Device,
        queue: &Queue,
        encoder: &mut wgpu::CommandEncoder,
    ) -> (Vec<egui::ClippedPrimitive>, ScreenDescriptor) {
        let full_output = self.ctx.end_frame();
        self.state.handle_platform_output(window, &self.ctx, full_output.platform_output);

        for (id, delta) in &full_output.textures_delta.set {
            self.renderer.update_texture(device, queue, *id, delta);
        }
        for id in &full_output.textures_delta.free {
            self.renderer.free_texture(id);
        }

        let paint_jobs = self.ctx.tessellate(full_output.shapes);
        let descriptor = ScreenDescriptor {
            size_in_pixels: window.inner_size().into(),
            pixels_per_point: window.scale_factor() as f32,
        };

        self.renderer.update_buffers(device, queue, encoder, &paint_jobs, &descriptor);
        (paint_jobs, descriptor)
    }

    ///! Renderiza las primitivas de `egui` en el `RenderPass` proporcionado.
    pub fn render<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        paint_jobs: &'a [egui::ClippedPrimitive],
        screen_descriptor: &'a ScreenDescriptor,
    ) {
        self.renderer.render(pass, paint_jobs, screen_descriptor);
    }

    /// Procesa un evento de ventana y lo pasa a egui_winit::State.
    pub fn on_event(
        &mut self,
        ctx: &egui::Context,
        event: &WindowEvent,
    ) -> egui_winit::EventResponse {
        self.state.on_event(ctx, event)
    }
}
