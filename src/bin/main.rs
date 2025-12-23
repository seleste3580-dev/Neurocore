use neurocore::{
    neuron::Sigmoid,
    synapse::Synapse,
    network::Network,
    learning::hebbian,
};

fn main() {
    println!("NeuroCore engine test starting…");

    // Create network
    let mut net = Network::new(Sigmoid);
    net.add_synapse(Synapse::new(0.8));
    net.add_synapse(Synapse::new(1.2));

    // Forward pass
    let input = 1.0;
    let output = net.forward(input).expect("network failed");

    println!("Input  : {}", input);
    println!("Output : {}", output);

    // Apply learning
    for syn in net.synapses_mut() {
        hebbian(syn, input, output, 0.05);
    }

    let new_output = net.forward(input).unwrap();
    println!("Output after learning: {}", new_output);

    println!("NeuroCore engine test completed ✔");
}
