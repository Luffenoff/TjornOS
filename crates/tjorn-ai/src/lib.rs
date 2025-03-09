pub mod ml;
pub mod nn;
pub mod optimizer;
pub mod dataset;
pub mod inference;
pub mod training;

pub use ml::MLEngine;
pub use nn::NeuralNetwork;
pub use inference::InferenceEngine;

pub fn init() {
    println!("Initializing {}", "tjorn-ai");
}
