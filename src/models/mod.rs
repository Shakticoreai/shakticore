pub mod loader;
pub mod converter;
pub mod registry;

use std::path::Path;
use anyhow::Result;

pub struct Model {
    // fields
}

impl Model {
    pub fn load<P: AsRef<Path>>(_path: P) -> Result<Self> {
        Ok(Self {})
    }
}
