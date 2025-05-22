use egui::Context;

/// Lógica de interfaz para el panel de diagnóstico.
pub fn draw_diagnostics(ctx: &Context, fps: f32, gpu_name: &str, clear_t: f32, width: u32, height: u32, format: &str, backend: &str) {
    egui::Window::new("Debug Info").show(ctx, |ui| {
        ui.label(format!("FPS: {:.1}", fps));
        ui.label(format!("GPU: {}", gpu_name));
        ui.label(format!("Clear Color t: {:.2}", clear_t));
        ui.label(format!("Resolución: {} x {}", width, height));
        ui.label(format!("Formato de superficie: {}", format));
        ui.label(format!("Backend: {}", backend));
    });
}
