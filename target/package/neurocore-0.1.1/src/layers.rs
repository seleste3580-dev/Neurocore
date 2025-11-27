use crate::{Tensor, Activation, NeuroError};
use serde::{Serialize, Deserialize};

/// Layer trait
pub trait Layer {
    fn forward(&self, input: &Tensor) -> Result<Tensor, NeuroError>;
    fn input_size(&self) -> usize;
    fn output_size(&self) -> usize;
}

/// Dense Layer
#[derive(Clone, Serialize, Deserialize)]
pub struct Dense {
    pub weights: Vec<Vec<f32>>,
    pub bias: Vec<f32>,
    pub activation: Activation,
}

impl Dense {
    pub fn new(input: usize, output: usize, activation: Activation) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let weights = (0..output)
            .map(|_| (0..input).map(|_| rng.gen_range(-0.5..0.5)).collect())
            .collect();

        let bias = vec![0.0; output];

        Self {
            weights,
            bias,
            activation,
        }
    }
}

impl Layer for Dense {
    fn forward(&self, input: &Tensor) -> Result<Tensor, NeuroError> {
        if input.len() != self.input_size() {
            return Err(NeuroError::ShapeMismatch {
                expected: self.input_size(),
                got: input.len(),
            });
        }

        let mut output = vec![0.0; self.output_size()];

        for (i, row) in self.weights.iter().enumerate() {
            let mut sum = 0.0;

            for (w, x) in row.iter().zip(input.data.iter()) {
                sum += w * x;
            }

            sum += self.bias[i];
            output[i] = self.activation.apply(sum);
        }

        Ok(Tensor::new(output))
    }

    fn input_size(&self) -> usize {
        self.weights[0].len()
    }

    fn output_size(&self) -> usize {
        self.bias.len()
    }
}
