use crate::render::renderer::Renderer;
use anyhow::Result;
use std::sync::Arc;
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
    /// Cuánto tarda el color de fondo en dar una vuelta completa.
    const COLOR_CYCLE_SECONDS: f32 = 12.0;

    /// Crea el núcleo del motor y el renderer asociado.
    pub async fn new(window: Arc<Window>) -> Result<Self> {
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
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;

        // El color avanza según el tiempo transcurrido, no según los fotogramas:
        // así tarda siempre lo mismo en dar una vuelta, vaya el equipo rápido o lento.
        self.clear_t =
            (self.clear_t + delta_time.as_secs_f32() / Self::COLOR_CYCLE_SECONDS) % 1.0;

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
    pub fn render(&mut self) -> Result<()> {
        self.renderer.render(self.clear_t)
    }

    /// FPS medidos en el último segundo.
    pub fn fps(&self) -> f32 {
        self.current_fps
    }

}
