use std::sync::Arc;

use anyhow::Result;
use nucleus::Engine;
use pollster::block_on;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

/// Estado de la aplicación. winit lo va llamando según lo que ocurra.
#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    engine: Option<Engine>,
}

impl ApplicationHandler for App {
    /// Se llama cuando la aplicación está lista para tener ventana.
    /// Puede llamarse más de una vez, así que se comprueba antes de crearla.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attributes = Window::default_attributes()
            .with_title("Demo con nucleus modularizado")
            .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0));

        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .expect("no pude crear la ventana"),
        );

        let engine = block_on(Engine::new(window.clone())).expect("falló Engine::new");

        self.window = Some(window);
        self.engine = Some(engine);
    }

    /// Un evento de la ventana: cerrar, redimensionar, redibujar...
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let (Some(window), Some(engine)) = (self.window.as_ref(), self.engine.as_mut()) else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => engine.resize(size.width, size.height),

            WindowEvent::RedrawRequested => {
                engine.update();

                if let Err(e) = engine.render() {
                    eprintln!("Error al renderizar: {e:?}");
                }

                // Pedir el siguiente fotograma: así el bucle sigue vivo.
                window.request_redraw();
            }

            _ => {}
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
