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
use winit::window::Window; // Ensure Window is imported

// Errores Genericos
use anyhow::Result;

use super::wgpu_context::WgpuContext;
use super::surface_manager::SurfaceManager;

// Egui
use egui::{Context as EguiContext, Visuals}; // FontData, FontDefinitions, FontFamily, FullOutput removed for now as font loading is commented
use egui_wgpu::Renderer as EguiWgpuRenderer;
use egui_winit::State as EguiWinitState;


// Estructura que agrupa Herramientas
pub struct Renderer {
    wgpu_context: WgpuContext,
    surface_manager: SurfaceManager,
    pub egui_ctx: EguiContext,
    pub egui_state: EguiWinitState,
    egui_wgpu_renderer: EguiWgpuRenderer,
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

        let egui_ctx = EguiContext::default();
       
        // Optional: Setup custom fonts (example)
        // let mut fonts = FontDefinitions::default();
        // fonts.font_data.insert("my_font".to_owned(), FontData::from_static(include_bytes!("../../assets/fonts/your_font.ttf"))); // Example path
        // fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());
        // egui_ctx.set_fonts(fonts);

        // Optional: Set dark theme (or light)
        egui_ctx.set_visuals(Visuals::dark());

        let scale_factor = window.scale_factor();
        let max_texture_side = wgpu_context.device.limits().max_texture_dimension_2d as usize;
        let mut egui_state = EguiWinitState::new(window);
        egui_state.set_max_texture_side(max_texture_side);
        egui_state.set_pixels_per_point(scale_factor as f32);
       
        let egui_wgpu_renderer = EguiWgpuRenderer::new(
            &wgpu_context.device,
            surface_manager.surface_format(), // Use the format from SurfaceManager
            None, // No depth format for egui overlay usually
            1,    // Sample count, usually 1 for egui
        );

        // Retorno del Renderer con todo listo para usar
        Ok(Self { wgpu_context, surface_manager, egui_ctx, egui_state, egui_wgpu_renderer })
    }

    /// Cambia tamaño de la superficie gráfica.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_manager.resize(&self.wgpu_context.device, width, height);
        // self.egui_state.set_max_texture_side(self.wgpu_context.device.limits().max_texture_dimension_2d as usize);
        // self.egui_state.set_pixels_per_point(scale_factor); // If scale factor can change
    }

    /// Dibuja un frame con un color variable.
    pub fn render(
        &mut self,
        clear_t: f32,
        window: &Window,
        fps: f32,        // New parameter
        gpu_name: &str,  // New parameter
    ) -> Result<()> { // SurfaceError was removed from Result by previous step, keeping as Result<()>
        let raw_input = self.egui_state.take_egui_input(window);
        self.egui_ctx.begin_frame(raw_input);

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
        
        // Create a simple egui UI
        egui::Window::new("Debug Info")
            .default_open(true)
            .show(&self.egui_ctx, |ui| {
                ui.label(format!("FPS: {:.1}", fps)); // Use passed fps
                ui.label(format!("GPU: {}", gpu_name)); // Use passed gpu_name
                ui.label(format!("Clear Color t: {:.2}", clear_t));
            });
        
        let full_output = self.egui_ctx.end_frame();
        self.egui_state.handle_platform_output(window, &self.egui_ctx, full_output.platform_output);
        
        let paint_jobs = self.egui_ctx.tessellate(full_output.shapes);

        // Crear encoder de comandos
        let mut encoder = self.wgpu_context.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Main Encoder"),
        });
        
        // Update egui wgpu renderer
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.surface_manager.width(), self.surface_manager.height()],
            pixels_per_point: window.scale_factor() as f32, // Use current scale factor
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_wgpu_renderer.update_texture(&self.wgpu_context.device, &self.wgpu_context.queue, *id, image_delta);
        }
        for id in &full_output.textures_delta.free {
            self.egui_wgpu_renderer.free_texture(id);
        }
       
        self.egui_wgpu_renderer.update_buffers(
            &self.wgpu_context.device,
            &self.wgpu_context.queue,
            &mut encoder, // Pass the command encoder here
            &paint_jobs,
            &screen_descriptor,
        );

        // Iniciar el render pass correctamente (esto asegura las transiciones necesarias)
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
            
            self.egui_wgpu_renderer.render(&mut render_pass, &paint_jobs, &screen_descriptor);
        }

        // Enviar comandos a la GPU y mostrar el resultado
        self.wgpu_context.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

      Ok(())
    }

}