use crate::tensor::Tensor;
use crate::layers::{Layer, DenseLayer};
use crate::activations::Activation;
use crate::loss::Loss;

pub struct NeuralNetwork {
    pub layers: Vec<Box<dyn Layer>>,
}

impl NeuralNetwork {
    pub fn new() -> Self {
        NeuralNetwork { layers: vec![] }
    }

    pub fn add_layer<L: Layer + 'static>(&mut self, layer: L) {
        self.layers.push(Box::new(layer));
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        let mut output = input.clone();
        for layer in &self.layers {
            output = layer.forward(&output);
        }
        output
    }

    // Training stub: you can extend backward propagation here
    pub fn train(&self, _input: &Tensor, _target: &Tensor, _loss: Loss, _epochs: usize, _lr: f64) {
        println!("Training method called: implement backward propagation and optimizer.");
    }
}
