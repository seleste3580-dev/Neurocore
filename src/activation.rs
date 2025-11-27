#[derive(Clone, Copy, Debug)]
pub enum Activation {
    None,
    ReLU,
    Sigmoid,
}

impl Activation {
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            Activation::None => x,
            Activation::ReLU => x.max(0.0),
            Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
        }
    }
}
