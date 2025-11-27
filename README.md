# 📘 Neurocore
### *A lightweight neural-state engine for Rust — created by **Seleste Waithaka**.*



## 🚀 Overview

**Neurocore** is a fast, lightweight, and extendable neural-state processing engine written in Rust.  
It provides simple building blocks for creating neural layers, activation functions, and forward-propagation logic — all without heavy machine-learning dependencies.

This crate focuses on **speed**, **minimalism**, and **clean architecture**, giving developers a flexible foundation for experimenting with neural computation in pure Rust.



## ✨ Features

- 🔹 Fully modular neural layers (Dense layers)
- 🔹 Built-in activation functions (ReLU, Sigmoid, Tanh, Linear)
- 🔹 Forward propagation engine
- 🔹 Serde serialization + deserialization
- 🔹 Safe, fast, 100% Rust implementation
- 🔹 Beginner-friendly, type-safe API
- 🔹 Zero heavy ML dependencies



## 📦 Installation

Add Neurocore to your project:

```bash
cargo add neuroflow
```

Or manually include it in `Cargo.toml`:

```toml
[dependencies]
neurocore = "0.1.0"
```



## 🧠 Example Usage

```rust
use neurocore::{Dense, Activation};

fn main() {
    // Create a Dense layer with 3 inputs and 2 outputs
    let layer = Dense::new(3, 2, Activation::Relu);

    // Example input vector
    let input = vec![1.0, 2.0, 3.0];

    // Perform forward propagation
    let output = layer.forward(&input);

    println!("Layer output: {:?}", output);
}
```



## 🧩 Architecture

Neurocore is built around three core components:

### ✔ 1. Activation Enum
Implements:
- ReLU  
- Sigmoid  
- Tanh  
- Linear  

Each with its own mathematical transformation.

### ✔ 2. Dense Layer
A fully connected layer with:
- Weight matrix  
- Bias vector  
- Activation function  
- Forward propagation logic  

### ✔ 3. Serialization
Neurocore includes automatic support for:

```rust
#[derive(Serialize, Deserialize)]
```

Allowing trained layers to be saved and loaded easily.



## ⚡ Performance

Neurocore is optimized for:
- minimal allocations  
- fast forward-pass execution  
- deterministic and stable results  
- tiny binary footprint  

The crate avoids any heavy machine-learning frameworks, making it ideal for embedded devices and performance-focused systems.


## 📚 Roadmap

Planned major updates:
- Multi-layer neural network (`Sequential`)
- Convolutional layer support
- GPU acceleration (WGPU)
- Training engine (SGD, ADAM)
- Dataset loader module
- Dropout and regularization tools



## 👑 Author

**Created by:**  
### **Waithaka Njoroge(seleste)**  
Rust developer • Machine learning enthusiast • Systems engineer



## 📄 License

This project is licensed under the **MIT License**.



## ⭐ Support the Project

If you find Neurocore helpful, consider:
- ⭐ Starring the GitHub repository  
- 🛠️ Contributing code  
- 💡 Suggesting new features  

Your support helps grow this project into a full ML engine written in Rust.
