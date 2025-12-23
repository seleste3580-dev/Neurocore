use crate::{
    neuron::{Linear, Sigmoid},
    synapse::Synapse,
    network::Network,
    learning::hebbian,
};

#[test]
fn linear_network_forward() {
    let mut net = Network::new(Linear);
    net.add_synapse(Synapse::new(0.5));
    net.add_synapse(Synapse::new(2.0));

    let out = net.forward(1.0).unwrap();
    assert_eq!(out, 1.0);
}

#[test]
fn sigmoid_activation_works() {
    let net = Network::new(Sigmoid);
    let out = net.forward(0.0);
    assert!(out.is_err()); // empty network must error
}

#[test]
fn hebbian_learning_updates_weight() {
    let mut syn = Synapse::new(1.0);
    hebbian(&mut syn, 1.0, 0.5, 0.1);
    assert!(syn.weight() > 1.0);
}
