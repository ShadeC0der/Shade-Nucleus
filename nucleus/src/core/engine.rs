use crate::render::renderer::Renderer;
use anyhow::Result;
use winit::window::Window;

/// Estructura principal del núcleo del motor.
pub struct Engine {
    pub renderer: Renderer, // Encargado de dibujar
    clear_t: f32,           // Lógica interna simple (color)
}

impl Engine {
    /// Crea el núcleo del motor y el renderer asociado.
    pub async fn new(window: &Window) -> Result<Self> {
        Ok(Self {
            renderer: Renderer::new(window).await?,
            clear_t: 0.0,
        })
    }

    /// Actualiza la lógica interna (por ahora, solo cambia un color).
    pub fn update(&mut self) {
        self.clear_t = (self.clear_t + 0.01) % 1.0;
    }

    /// Ajusta tamaño del área gráfica cuando cambia la ventana.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    /// Solicita al renderer que dibuje el frame actual.
    pub fn render(&mut self) -> Result<()> {
        self.renderer.render(self.clear_t)
    }
}
