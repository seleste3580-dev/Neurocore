pub trait Neuron: Send + Sync {
    fn activate(&self, input: f64) -> f64;
}

#[derive(Debug, Default)]
pub struct Linear;

impl Neuron for Linear {
    fn activate(&self, input: f64) -> f64 {
        input
    }
}

#[derive(Debug)]
pub struct Sigmoid;

impl Neuron for Sigmoid {
    fn activate(&self, input: f64) -> f64 {
        1.0 / (1.0 + (-input).exp())
    }
}
