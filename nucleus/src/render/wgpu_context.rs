// Manejo de Errores
use anyhow::Result;
// Importaciones WGPU
use wgpu::{
    Adapter, Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference, Queue,
    RequestAdapterOptions,
};

use crate::utils::messages::{DEVICE_CREATION_ERROR, GPU_ADAPTER_ERROR};

/// Encapsula los componentes principales de WGPU: Instance, Adapter, Device, and Queue.
pub struct WgpuContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

impl WgpuContext {
    /// Crea un nuevo WgpuContext. Esto implica inicializar WGPU solicitando un adaptador y un dispositivo.
    pub async fn new() -> Result<Self> {
        let instance = Instance::default(); // Crea una instancia de WGPU.

        // Solicita un adaptador (GPU).
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance, // Preferencia de potencia alta
                compatible_surface: None, // Superficie no utilizada por ahora
                force_fallback_adapter: false, // No forzar un adaptador de reserva
                ..Default::default()
            })
            .await
            .map_err(|_| anyhow::anyhow!(GPU_ADAPTER_ERROR))?;

        // Solicita un dispositivo y una cola del adaptador.
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("WGPU Device"),           // Etiqueta del dispositivo
                required_features: Features::empty(), // Características del dispositivo
                required_limits: Limits::default(),   // Límites del dispositivo
                ..Default::default()
            })
            .await
            .map_err(|_| anyhow::anyhow!(DEVICE_CREATION_ERROR))?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
}
