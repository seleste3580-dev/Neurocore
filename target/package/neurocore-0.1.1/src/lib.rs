//! Neuroflow: real‑time neural computation pipelines for embedded & edge devices

pub mod layers;
pub mod activation;
pub mod pipeline;
pub mod errors;
pub mod tensor;

pub use layers::*;
pub use activation::*;
pub use pipeline::*;
pub use errors::*;
pub use tensor::*;
