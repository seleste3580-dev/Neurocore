use crate::layers::{Layer, DenseLayer};
use crate::tensor::Tensor;

pub struct NeuralNetwork {
    pub layers: Vec<Box<dyn Layer>>,
}

impl NeuralNetwork {
    pub fn new() -> Self {
        NeuralNetwork { layers: vec![] }
    }

    pub fn add_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        let mut x = input.clone();
        for layer in &self.layers {
            x = layer.forward(&x);
        }
        x
    }
}
