//! renderer.rs
//! 
//! Este módulo define el struct `Renderer`, que se encarga de inicializar y controlar 
//! los recursos gráficos necesarios para mostrar algo en pantalla usando `wgpu`. 
//! 
//! Su responsabilidad principal es:
//! - Configurar la conexión entre la GPU (`Device`, `Queue`) y la ventana (`Surface`).
//! - Definir cómo se presenta el contenido en pantalla (`SurfaceConfiguration`).
//! - Proveer funciones para redimensionar y renderizar el contenido.
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
    Device,
    DeviceDescriptor,
    Instance,
    Features,
    Limits,
    PowerPreference, 
    Queue,
    RequestAdapterOptions, 
    TextureUsages,
    Surface, 
    SurfaceConfiguration,
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

// Estructura que agrupa Herramientas
pub struct Renderer {
    surface: Surface,                   // Área visible en pantalla            
    device: Device,                     // Interfaz con la GPU            
    queue: Queue,                       // Cola de comandos           
    config: SurfaceConfiguration,       // Configuración de render
}

impl Renderer {
    /// Inicializa el renderizador (GPU + área de dibujo).
    pub async fn new(window: &Window) -> Result<Self> {
        // Crear la instancia con validación habilitada
        let instance = Instance::default();

        // Crear la superficie (conexión con la ventana)
        let surface = unsafe { instance.create_surface(window)? };

        // Busca un adaptador (GPU)
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            // Criterio de busqueda
            compatible_surface: Some(&surface),                 // Asegura Compativilidad con la ventana
            power_preference: PowerPreference::HighPerformance, // Elige la mas potente GPU
            force_fallback_adapter: false,                      // Solo acepta GPU compatible sino se cierra

        }).await.ok_or_else(|| anyhow::anyhow!("No se encontró una GPU compatible"))?;

        // Conectar el dispositivo (GPU)
        let (device, queue) = adapter.request_device(&DeviceDescriptor {
                label: Some("Main Device"),     // Nombre de depuración
                features: Features::empty(),    // No se pide nada avanzado
                limits: Limits::default(),      // Se usan límites básicos (máxima compatibilidad)
            }, None).await?;

        // Obtener el tamaño de la ventana
        let size = window.inner_size();

        // Obtener el formato de color compatible
        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats.iter().copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        // Configurar la superficie
        // FUTURE: Aquí podríamos usar distintos perfiles de renderizado:
        // Por ejemplo: Low, Medium, High → cambiando format, present_mode, etc.
        // A futuro engine deberia controlar esta configuracion
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT, // Uso del Surface
            format,                                  // Formato de Colores
            width: size.width,                       // Ancho en px
            height: size.height,                     // Largo en px
            present_mode: caps.present_modes[0],     // Cómo se muestran los frames
            alpha_mode: caps.alpha_modes[0],         // Cómo trata la transparencia
            view_formats: vec![],                    // Vista adicional para texturas 
        };

        // Aplica Configuracion en el Surface
        surface.configure(&device, &config);

        // Retorno del Renderer con todo listo para usar
        Ok(Self { surface, device, queue, config})
    }

    /// Cambia tamaño de la superficie gráfica.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Dibuja un frame con un color variable.
    pub fn render(&mut self, clear_t: f32) -> Result<()> {
        // Obtener el frame actual
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        // Calcular el color del fondo
        let color = Color {
            r: clear_t as f64,
            g: (1.0 - clear_t) as f64,
            b: (clear_t * 0.5) as f64,
            a: 1.0,
        };

        // Crear encoder de comandos
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
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
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

      Ok(())
    }

}