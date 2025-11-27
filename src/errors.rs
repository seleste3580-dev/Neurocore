use thiserror::Error;

#[derive(Debug, Error)]
pub enum NeuroError {
    #[error("Shape mismatch: expected {expected}, got {got}")]
    ShapeMismatch { expected: usize, got: usize },

    #[error("Pipeline has no layers")]
    EmptyPipeline,

    #[error("Runtime error: {0}")]
    Runtime(String),
}
