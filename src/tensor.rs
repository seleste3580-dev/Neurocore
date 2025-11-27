#[derive(Clone, Debug)]
pub struct Tensor {
    pub data: Vec<f64>,
}

impl Tensor {
    pub fn new(data: Vec<f64>) -> Self {
        Tensor { data }
    }

    pub fn from_vec(data: Vec<f64>) -> Self {
        Tensor { data }
    }
}
