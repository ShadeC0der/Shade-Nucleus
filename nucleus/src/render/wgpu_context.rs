use anyhow::Result;
use wgpu::{
    Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference, Queue,
    RequestAdapterOptions, Adapter,
};
use winit::window::Window;

/// Encapsulates WGPU's core components: Instance, Adapter, Device, and Queue.
pub struct WgpuContext {
    #[allow(dead_code)] // Instance is not read after creation currently, but is vital
    pub instance: Instance,
    #[allow(dead_code)] // Adapter is not read after creation currently, but is vital
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

impl WgpuContext {
    /// Creates a new WgpuContext.
    /// This involves initializing WGPU by requesting an adapter and a device.
    pub async fn new() -> Result<Self> {
        // Create the instance with default backends and DX12 shader compiler if available.
        let instance = Instance::default();

        // Request an adapter (GPU).
        // This is an asynchronous operation.
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: None, // We'll create surface later in Renderer
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find a suitable GPU adapter."))?;

        // Request a device and queue from the adapter.
        // This is also an asynchronous operation.
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("WGPU Device"),
                    features: Features::empty(), // Use default features
                    limits: Limits::default(),   // Use default limits
                },
                None, // Trace path, not needed for now
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create WGPU device and queue: {}", e))?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
}
