// Manejo de Errores
use anyhow::Result;
// Importaciones necesarias para WGPU
use wgpu::{
    Adapter, CurrentSurfaceTexture, Device, Instance, PresentMode, Surface, SurfaceColorSpace,
    SurfaceConfiguration, SurfaceTexture, TextureFormat, TextureUsages,
};
// Ventana de Winit
use std::sync::Arc;
use winit::window::Window;

/// Administra la superficie WGPU y su configuración.
pub struct SurfaceManager {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
}

/// Implementa la funcionalidad de la superficie WGPU.
impl SurfaceManager {
    /// Inicializa la superficie WGPU y la configura para la ventana dada.
    pub fn new(
        window: Arc<Window>,
        instance: &Instance,
        adapter: &Adapter,
        device: &Device,
    ) -> Result<Self> {
        // Crear la superficie WGPU a partir de la ventana.
        // La superficie se queda con una copia del Arc, así que la ventana no
        // puede morir antes que ella: por eso ya no hace falta `unsafe`.
        let surface = instance.create_surface(window.clone())?;

        // Obtener el tamaño de la ventana
        let size = window.inner_size();

        // Obtener las capacidades de la superficie
        let caps = surface.get_capabilities(adapter);

        // Elegir el formato de textura adecuado
        let format = caps
            .formats // Formatos de textura soportados
            .iter() // Iterar sobre los formatos
            .copied() // Copiar los formatos
            .find(|f| f.is_srgb()) // Buscar un formato sRGB
            .unwrap_or(caps.formats[0]); // Si no se encuentra, usar el primero

        // Configurar la superficie
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT, // Uso de la textura
            format,                                  // Formato de la textura
            width: size.width.max(1),                // Ancho mínimo de 1
            height: size.height.max(1),              // Largo minimo de 1
            present_mode: PresentMode::Fifo,         // Sincronizado con la pantalla
            alpha_mode: caps.alpha_modes[0],         // Modo alfa
            view_formats: vec![],                    // Formatos de vista
            color_space: SurfaceColorSpace::Auto,
            desired_maximum_frame_latency: 2,
        };

        // Aplica la configuración a la superficie
        surface.configure(device, &config);

        // Devuelve la instancia de SurfaceManager y la configuración de la superficie
        Ok(Self { surface, config })
    }

    /// Cambia el tamaño de la superficie según las nuevas dimensiones.
    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width; // Asegúrate de que el ancho sea mayor que 0
            self.config.height = height; // Asegúrate de que la altura sea mayor que 0
            self.surface.configure(device, &self.config); // Reconfigurar superficie con las nuevas dimensiones
        }
    }

    /// Recupera la textura actual de la superficie para ser renderizada.
    pub fn get_current_texture(&self) -> Result<SurfaceTexture> {
        match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture)
            | CurrentSurfaceTexture::Suboptimal(texture) => Ok(texture),
            otro => Err(anyhow::anyhow!("no se pudo obtener la textura: {otro:?}")),
        }
    }

    /// Devuelve el formato de textura de la superficie.
    pub fn surface_format(&self) -> TextureFormat {
        self.config.format
    }

    /// Devuelve el ancho de la superficie WGPU
    pub fn width(&self) -> u32 {
        self.config.width
    }

    /// Devuelve la altura de la superficie WGPU.
    pub fn height(&self) -> u32 {
        self.config.height
    }
}

