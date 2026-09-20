use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use anyhow::Context;
use nucleus::Engine;
use pollster::block_on;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo con nucleus modularizado")
        .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0))
        .build(&event_loop)
        .context("no pude crear la ventana")?;

    let mut engine = block_on(Engine::new(&window))
        .context("falló Engine::new")?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested =>
                        *control_flow = ControlFlow::ExitWithCode(0),

                    WindowEvent::Resized(sz) =>
                        engine.resize(sz.width, sz.height),

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>
                        engine.resize(new_inner_size.width, new_inner_size.height),

                    _ => {}
                }
            }

            Event::MainEventsCleared => {
                engine.update();

                if let Err(e) = engine.render() {
                    eprintln!("Error al renderizar: {:?}", e);

                    if let Some(wgpu::SurfaceError::Lost) = e.downcast_ref::<wgpu::SurfaceError>() {
                        engine.resize(
                            engine.renderer.surface_width(),
                            engine.renderer.surface_height(),
                        );
                    } else if let Some(wgpu::SurfaceError::OutOfMemory) =
                        e.downcast_ref::<wgpu::SurfaceError>()
                    {
                        *control_flow = ControlFlow::ExitWithCode(1);
                    }
                }
            }

            _ => {}
        }
    });
}
