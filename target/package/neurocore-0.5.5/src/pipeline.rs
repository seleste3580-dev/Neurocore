use crate::{Layer, Tensor, NeuroError};

pub struct Pipeline {
    layers: Vec<Box<dyn Layer + Send + Sync>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { layers: vec![] }
    }

    pub fn add_layer<L: Layer + Send + Sync + 'static>(&mut self, l: L) -> Result<(), NeuroError> {
        if let Some(prev) = self.layers.last() {
            if prev.output_size() != l.input_size() {
                return Err(NeuroError::ShapeMismatch {
                    expected: prev.output_size(),
                    got: l.input_size(),
                });
            }
        }

        self.layers.push(Box::new(l));
        Ok(())
    }

    pub fn run(&self, input: Tensor) -> Result<Tensor, NeuroError> {
        if self.layers.is_empty() {
            return Err(NeuroError::EmptyPipeline);
        }

        let mut result = input;

        for layer in &self.layers {
            result = layer.forward(&result)?;
        }

        Ok(result)
    }
}
