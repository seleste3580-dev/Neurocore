use crate::tensor::Tensor;
use crate::activations::Activation;
use rand::Rng;

pub trait Layer {
    fn forward(&self, input: &Tensor) -> Tensor;
    fn input_size(&self) -> usize;
    fn output_size(&self) -> usize;
}

pub struct DenseLayer {
    pub input_size: usize,
    pub output_size: usize,
    pub weights: Vec<Vec<f64>>,
    pub biases: Vec<f64>,
    pub activation: Activation,
}

impl DenseLayer {
    pub fn new(input_size: usize, output_size: usize, activation: Activation) -> Self {
        let mut rng = rand::thread_rng();
        let weights = (0..output_size)
            .map(|_| (0..input_size).map(|_| rng.gen::<f64>() * 0.1).collect())
            .collect();
        let biases = vec![0.0; output_size];

        DenseLayer { input_size, output_size, weights, biases, activation }
    }
}

impl Layer for DenseLayer {
    fn forward(&self, input: &Tensor) -> Tensor {
        let mut output = vec![0.0; self.output_size];
        for i in 0..self.output_size {
            for j in 0..self.input_size {
                output[i] += self.weights[i][j] * input.data[j];
            }
            output[i] += self.biases[i];
            output[i] = self.activation.apply(output[i]);
        }
        Tensor::new(output, vec![self.output_size])
    }

    fn input_size(&self) -> usize { self.input_size }
    fn output_size(&self) -> usize { self.output_size }
}
