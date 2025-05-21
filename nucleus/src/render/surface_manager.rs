use anyhow::Result;
use wgpu::{
    Adapter, Device, Instance, Surface, SurfaceConfiguration, SurfaceError,
    SurfaceTexture, TextureUsages, TextureFormat,
};
use winit::window::Window;

/// Manages the WGPU Surface and its configuration.
pub struct SurfaceManager {
    surface: Surface,
    config: SurfaceConfiguration,
}

impl SurfaceManager {
    /// Creates a new SurfaceManager.
    /// Initializes the WGPU surface and configures it for the given window.
    pub fn new(
        window: &Window,
        instance: &Instance,
        adapter: &Adapter,
        device: &Device,
    ) -> Result<Self> {
        let surface = unsafe { instance.create_surface(window)? };

        let size = window.inner_size();
        let caps = surface.get_capabilities(adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1), // Ensure non-zero width
            height: size.height.max(1), // Ensure non-zero height
            present_mode: caps.present_modes[0], // Vsync, typically Fifo
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(device, &config);

        Ok(Self { surface, config })
    }

    /// Resizes the surface based on new dimensions.
    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(device, &self.config);
        }
    }

    /// Retrieves the current texture from the surface to be rendered onto.
    pub fn get_current_texture(&self) -> Result<SurfaceTexture, SurfaceError> {
        self.surface.get_current_texture()
    }

    /// Returns the texture format of the surface.
    #[allow(dead_code)]
    pub fn surface_format(&self) -> TextureFormat {
        self.config.format
    }

    pub fn width(&self) -> u32 {
        self.config.width
    }

    pub fn height(&self) -> u32 {
        self.config.height
    }
}
