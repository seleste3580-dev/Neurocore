pub enum Activation {
    None,
    ReLU,
    Sigmoid,
    Tanh,
    Softmax,
}

impl Activation {
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            Activation::None => x,
            Activation::ReLU => if x > 0.0 { x } else { 0.0 },
            Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            Activation::Tanh => x.tanh(),
            Activation::Softmax => x, // Softmax handled on vectors in network
        }
    }

    pub fn derivative(&self, x: f64) -> f64 {
        match self {
            Activation::None => 1.0,
            Activation::ReLU => if x > 0.0 { 1.0 } else { 0.0 },
            Activation::Sigmoid => {
                let sig = 1.0 / (1.0 + (-x).exp());
                sig * (1.0 - sig)
            },
            Activation::Tanh => 1.0 - x.tanh().powi(2),
            Activation::Softmax => 1.0, // Not used individually
        }
    }
}
