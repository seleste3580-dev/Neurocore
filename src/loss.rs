pub enum Loss {
    MSE,
    CrossEntropy,
}

impl Loss {
    pub fn compute(&self, predicted: &[f64], target: &[f64]) -> f64 {
        match self {
            Loss::MSE => predicted.iter().zip(target.iter())
                                  .map(|(p, t)| (p - t).powi(2))
                                  .sum::<f64>() / predicted.len() as f64,
            Loss::CrossEntropy => -predicted.iter().zip(target.iter())
                                  .map(|(p, t)| t * p.ln())
                                  .sum::<f64>() / predicted.len() as f64,
        }
    }

    pub fn derivative(&self, predicted: &[f64], target: &[f64]) -> Vec<f64> {
        match self {
            Loss::MSE => predicted.iter().zip(target.iter())
                                   .map(|(p, t)| 2.0 * (p - t) / predicted.len() as f64)
                                   .collect(),
            Loss::CrossEntropy => predicted.iter().zip(target.iter())
                                           .map(|(p, t)| -(t / p))
                                           .collect(),
        }
    }
}
