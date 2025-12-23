use crate::synapse::Synapse;

/// Hebbian learning rule
/// Δw = η * input * output
pub fn hebbian(
    synapse: &mut Synapse,
    input: f64,
    output: f64,
    rate: f64,
) {
    let delta = rate * input * output;
    synapse.adjust(delta);
}
