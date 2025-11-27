use neurocore::*;

fn main() {
    let mut nn = NeuralNetwork::new();

    nn.add_layer(DenseLayer::new(3, 5, Activation::ReLU));
    nn.add_layer(DenseLayer::new(5, 2, Activation::Sigmoid));

    let input = Tensor::new(vec![0.5, -0.3, 0.8], vec![3]);
    let output = nn.forward(&input);

    println!("Output: {:?}", output.data);

    let target = Tensor::new(vec![1.0, 0.0], vec![2]);
    nn.train(&input, &target, Loss::MSE, 10, 0.01);
}
