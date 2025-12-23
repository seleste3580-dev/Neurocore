//! NeuroCore
//!
//! A deterministic neural computation engine.
//! Provides primitives for neurons, synapses, networks, and learning rules.

pub mod error;
pub mod neuron;
pub mod synapse;
pub mod network;
pub mod learning;

#[cfg(test)]
mod tests;
