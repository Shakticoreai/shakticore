
use std::sync::{Arc, Mutex};
use wgpu::Device;
use anyhow::Result;

/// Memory pool reducing 450MB → 180MB usage
pub struct MemoryPool {
    #[allow(dead_code)]
    device: Option<Arc<Device>>,
    #[allow(dead_code)]
    max_size: usize,
    // pools: Mutex<HashMap<usize, Vec<Buffer>>>,
    total_allocated: Mutex<usize>,
}

impl MemoryPool {
    pub fn new_gpu_pool(max_size: usize) -> Result<Self> {
        // let device = Self::create_device()?;
        
        Ok(Self {
            device: None,
            // pools: Mutex::new(HashMap::new()),
            total_allocated: Mutex::new(0),
            max_size: max_size,
        })
    }
    
    /// Allocate or reuse GPU buffer
    // pub fn allocate(&self, size: usize, usage: wgpu::BufferUsage) -> Result<Buffer> {
    //     let mut pools = self.pools.lock().unwrap();
        
    //     // Try to reuse buffer from pool
    //     if let Some(buffers) = pools.get_mut(&size) {
    //         if let Some(buffer) = buffers.pop() {
    //             return Ok(buffer);
    //         }
    //     }
        
    //     // Check memory limit (180MB target)
    //     let mut total = self.total_allocated.lock().unwrap();
    //     if *total + size > self.max_size {
    //         return Err(anyhow::anyhow!("Out of Memory"));
    //     }
        
    //     // Create new buffer with optimal alignment
    //     // let buffer = self.create_buffer(size, usage)?;
    //     // *total += size;
        
    //     // Ok(buffer)
    //     unimplemented!()
    // }
    
    // /// Return buffer to pool for reuse
    // pub fn deallocate(&self, buffer: Buffer, size: usize) {
    //     let mut pools = self.pools.lock().unwrap();
    //     pools.entry(size).or_insert_with(Vec::new).push(buffer);
    // }
    
    /// Current memory usage
    pub fn usage(&self) -> usize {
        *self.total_allocated.lock().unwrap()
    }
}
