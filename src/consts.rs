// Window setup
pub const WINDOW_WIDTH: f32 = 1366.;
pub const WINDOW_HEIGHT: f32 = 720.;

// Dataset
pub(crate) const DATASET_PATH: &str = "data/eil51.tsp.txt";
// pub(crate) const DATASET_PATH: &str = "data/kroA100.tsp.txt";

// Simulation
pub const STARTING_UPS: f64 = 1. / 60.;
pub const ITERATIONS: usize = 1_0000;

// Simulated Annealing
pub const STARTING_TEMPERATURE: f32 = 1.0;
pub const MIN_TEMP: f32 = 0.000_001;
