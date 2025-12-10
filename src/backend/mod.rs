pub mod gpu;
pub mod cpu;
pub mod hybrid;


use anyhow::Result;

pub enum ComputeBackend {
    GPU(gpu::vulkan::VulkanBackend),
    CUDA(gpu::cuda::CudaBackend),
    CPU(cpu::simd::SimdBackend),
}

impl ComputeBackend {
    pub fn auto_detect() -> Result<Self> {
        // Simple mock implementation for now
        // In reality, check for available hardware
        Ok(ComputeBackend::GPU(gpu::vulkan::VulkanBackend::new()?))
    }
}
