

#[derive(Clone, Debug)]
pub struct Tensor {
    shape: Vec<usize>,
    data: Vec<f32>,
}

impl Tensor {
    pub fn new(shape: &[usize], data: Vec<f32>) -> Self {
        Self {
            shape: shape.to_vec(),
            data,
        }
    }

    pub fn random(shape: &[usize]) -> Self {
        // Mock random data
        let size: usize = shape.iter().product();
        Self {
            shape: shape.to_vec(),
            data: vec![0.0; size],
        }
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn memory_usage(&self) -> usize {
        self.data.len() * std::mem::size_of::<f32>()
    }
}
