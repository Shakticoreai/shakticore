use std::time::Duration;

pub struct MetricsCollector;

impl MetricsCollector {
    pub fn new() -> Self {
        Self
    }
    
    pub fn record_inference(&self, _duration: Duration, _input_mem: usize, _output_mem: usize) {
        // TODO: Record metrics
    }
}
