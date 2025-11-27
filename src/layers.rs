use crate::tensor::Tensor;
use crate::activation::Activation;

pub trait Layer {
    fn forward(&self, input: &Tensor) -> Tensor;
    fn input_size(&self) -> usize;
    fn output_size(&self) -> usize;
}

pub struct DenseLayer {
    input_size: usize,
    output_size: usize,
    activation: Activation,
    weights: Vec<Vec<f64>>,
    biases: Vec<f64>,
}

impl DenseLayer {
    pub fn new(input_size: usize, output_size: usize, activation: Activation) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let weights = (0..output_size)
            .map(|_| (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect())
            .collect();
        let biases = (0..output_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
        DenseLayer {
            input_size,
            output_size,
            activation,
            weights,
            biases,
        }
    }
}

impl Layer for DenseLayer {
    fn forward(&self, input: &Tensor) -> Tensor {
        let mut output = Vec::new();
        for i in 0..self.output_size {
            let mut sum = self.biases[i];
            for j in 0..self.input_size {
                sum += self.weights[i][j] * input.data[j];
            }
            output.push(self.activation.apply(sum));
        }
        Tensor::new(output)
    }

    fn input_size(&self) -> usize {
        self.input_size
    }

    fn output_size(&self) -> usize {
        self.output_size
    }
}
