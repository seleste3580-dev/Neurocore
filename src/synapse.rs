#[derive(Debug, Clone)]
pub struct Synapse {
    weight: f64,
}

impl Synapse {
    pub fn new(weight: f64) -> Self {
        Self { weight }
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn transmit(&self, signal: f64) -> f64 {
        signal * self.weight
    }

    pub fn adjust(&mut self, delta: f64) {
        self.weight += delta;
    }
}
