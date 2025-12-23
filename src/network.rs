use crate::{neuron::Neuron, synapse::Synapse, error::NeuroError};

pub struct Network<N: Neuron> {
    neuron: N,
    synapses: Vec<Synapse>,
}

impl<N: Neuron> Network<N> {
    pub fn new(neuron: N) -> Self {
        Self {
            neuron,
            synapses: Vec::new(),
        }
    }

    pub fn add_synapse(&mut self, synapse: Synapse) {
        self.synapses.push(synapse);
    }

    pub fn forward(&self, input: f64) -> Result<f64, NeuroError> {
        if self.synapses.is_empty() {
            return Err(NeuroError::EmptyNetwork);
        }

        let mut signal = input;
        for syn in &self.synapses {
            signal = syn.transmit(signal);
        }

        Ok(self.neuron.activate(signal))
    }

    pub fn synapses_mut(&mut self) -> &mut [Synapse] {
        &mut self.synapses
    }
}
