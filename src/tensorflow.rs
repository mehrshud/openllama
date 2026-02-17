use log::info;
use src::models::{User, EducationInstitution};
use src::config::Config;
use src::database::DB;
use tensorflow::{Graph, Session, SessionOptions};

pub struct TensorFlowService {
    config: Config,
    db: DB,
}

impl TensorFlowService {
    pub fn new(config: Config, db: DB) -> Self {
        TensorFlowService { config, db }
    }

    /// Load a saved TensorFlow model from a file
    pub fn load_model(&self, model_path: &str) -> Result<Graph, String> {
        let mut graph = Graph::new();
        match graph.import_graph_def(&std::fs::read(model_path).unwrap(), &SessionOptions::new()) {
            Ok(_) => {
                info!("Loaded TensorFlow model from {}", model_path);
                Ok(graph)
            },
            Err(e) => {
                info!("Error loading TensorFlow model: {}", e);
                Err(format!("Error loading TensorFlow model: {}", e))
            }
        }
    }

    /// Use the loaded model to make a prediction
    pub fn make_prediction(&self, model: &Graph, input_data: &User) -> Result<f32, String> {
        let mut session = Session::new(&SessionOptions::new(), &model).unwrap();
        let output = session.run(
            &[],
            &[
                ("input:0", &tensorflow::Tensor::new(&[1, 3], tensorflow::DT_FLOAT)),
            ],
            &[
                "output:0",
            ],
            &[
                ("input:0", &tensorflow::Tensor::new(&[1, 3], tensorflow::DT_FLOAT)),
            ],
        ).unwrap();
        if let Some(output) = output.get(0) {
            let output: f32 = output.unwrap();
            info!("Made prediction using TensorFlow model: {}", output);
            Ok(output)
        } else {
            info!("Error making prediction: no output found");
            Err("Error making prediction: no output found".to_string())
        }
    }
}