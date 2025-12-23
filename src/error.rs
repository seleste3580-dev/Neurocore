use std::fmt;

#[derive(Debug)]
pub enum NeuroError {
    EmptyNetwork,
    InvalidWeight,
}

impl fmt::Display for NeuroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NeuroError::EmptyNetwork => write!(f, "network contains no synapses"),
            NeuroError::InvalidWeight => write!(f, "invalid synapse weight"),
        }
    }
}

impl std::error::Error for NeuroError {}
