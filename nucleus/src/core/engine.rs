use crate::render::renderer::Renderer;
use anyhow::Result;
use winit::window::Window;
use std::time::{Duration, Instant};

/// Estructura principal del núcleo del motor.
pub struct Engine {
    pub renderer: Renderer, // Encargado de dibujar
    clear_t: f32,           // Lógica interna simple (color)
    last_frame_time: Instant,
    frame_count: u32,
    fps_update_timer: Duration,
    current_fps: f32,
}

impl Engine {
    /// Crea el núcleo del motor y el renderer asociado.
    pub async fn new(window: &Window) -> Result<Self> {
        Ok(Self {
            renderer: Renderer::new(window).await?,
            clear_t: 0.0,
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps_update_timer: Duration::ZERO,
            current_fps: 0.0,
        })
    }

    /// Actualiza la lógica interna (por ahora, solo cambia un color).
    pub fn update(&mut self) {
        self.clear_t = (self.clear_t + 0.01) % 1.0;

        let now = Instant::now();
        let delta_time = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;

        self.frame_count += 1;
        self.fps_update_timer += delta_time;

        if self.fps_update_timer >= Duration::from_secs(1) {
            self.current_fps = self.frame_count as f32 / self.fps_update_timer.as_secs_f32();
            self.frame_count = 0;
            self.fps_update_timer -= Duration::from_secs(1);
        }
    }

    /// Ajusta tamaño del área gráfica cuando cambia la ventana.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    /// Solicita al renderer que dibuje el frame actual.
    pub fn render(&mut self, window: &Window) -> Result<()> {
        let gpu_name = self.renderer.gpu_name(); // ✅ Usamos el método público
        self.renderer.render(self.clear_t, window, self.current_fps, &gpu_name)
    }

}
