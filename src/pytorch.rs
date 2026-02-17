use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use log::info;
use pytorch::{Module, Tensor};

/// PyTorch implementation for OpenLlama
pub struct PyTorchImpl {
    model: Module,
    config: Config,
}

impl PyTorchImpl {
    /// Initializes the PyTorch implementation with a given configuration
    pub fn new(config: Config) -> PyTorchImpl {
        PyTorchImpl {
            model: Module::new(),
            config,
        }
    }

    /// Trains the PyTorch model with user data
    pub fn train(&mut self, user: &User) -> Result<(), String> {
        info!("Training PyTorch model with user data");
        let input = Tensor::from_slice(&[user.id as f64]);
        match self.model.forward_t(&input) {
            Ok(output) => {
                info!("Model output: {:?}", output);
                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }

    /// Evaluates the PyTorch model with user data
    pub fn evaluate(&self, user: &User) -> Result<f64, String> {
        info!("Evaluating PyTorch model with user data");
        let input = Tensor::from_slice(&[user.id as f64]);
        match self.model.forward_t(&input) {
            Ok(output) => {
                info!("Model output: {:?}", output);
                Ok(output.into_tensor().mean().unwrap())
            }
            Err(err) => Err(err.to_string()),
        }
    }
}
