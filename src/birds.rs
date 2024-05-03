use serde::{Serialize, Deserialize};
use image::{DynamicImage};

pub use crate::simulation::*;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 800;
const DEFAULT_MAX_ITER: u32 = 1000;
const DEFAULT_N_BIRDS: u32 = 50;
const DEFAULT_N_ITER: u32 = 1000;

#[derive(Debug, Serialize, Deserialize)]
pub struct BirdSimulationParameters {
    pub width: u32,
    pub height: u32,
    pub max_iter: u32,
    pub n_birds: u32,
    pub n_iter: u32
}

impl std::default::Default for BirdSimulationParameters {
    fn default() -> Self {
        BirdSimulationParameters {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            max_iter: DEFAULT_MAX_ITER,
            n_birds: DEFAULT_N_BIRDS,
            n_iter: DEFAULT_N_ITER
        }
    }
}

impl BirdSimulationParameters {
    pub fn parameter_descriptions() -> Vec<ParameterDescription> {
        vec![
            ParameterDescription {
                name: "width".to_string(),
                description: "Width of the image".to_string(),
                data_type: "integer".to_string(),
                default: ParameterType::UnsignedInteger(DEFAULT_WIDTH),
                required: true,
                lower_bound: Some(ParameterType::Integer(0)),
                upper_bound: None,
                step: Some(ParameterType::Integer(1))
            },
            ParameterDescription {
                name: "height".to_string(),
                description: "Height of the image".to_string(),
                data_type: "integer".to_string(),
                default: ParameterType::UnsignedInteger(DEFAULT_HEIGHT),
                required: true,
                lower_bound: Some(ParameterType::Integer(0)),
                upper_bound: None,
                step: Some(ParameterType::Integer(1))
            },
            ParameterDescription {
                name: "max_iter".to_string(),
                description: "Maximum number of iterations".to_string(),
                data_type: "integer".to_string(),
                default: ParameterType::UnsignedInteger(DEFAULT_MAX_ITER),
                required: true,
                lower_bound: Some(ParameterType::Integer(10)),
                upper_bound: Some(ParameterType::Integer(10000)),
                step: Some(ParameterType::Integer(10))
            },
            ParameterDescription {
                name: "n_birds".to_string(),
                description: "The number of birds to be used in the simulation".to_string(),
                data_type: "integer".to_string(),
                default: ParameterType::UnsignedInteger(DEFAULT_N_BIRDS),
                required: true,
                lower_bound: Some(ParameterType::Integer(0)),
                upper_bound: Some(ParameterType::Integer(1000)),
                step: Some(ParameterType::Integer(1))
            }
        ]
    }

    pub fn from_parameters(&mut self, params: SimulationParameters) {
        for p in params.params {
            match p.name.as_str() {
                "width" => {
                    if let ParameterType::UnsignedInteger(value) = p.value {
                        self.width = value;
                    }
                }
                "height" => {
                    if let ParameterType::UnsignedInteger(value) = p.value {
                        self.height = value;
                    }
                }
                "max_iter" => {
                    if let ParameterType::UnsignedInteger(value) = p.value {
                        self.max_iter = value;
                    }
                }
                "n_birds" => {
                    if let ParameterType::UnsignedInteger(value) = p.value {
                        self.n_iter = value;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn run(&self) -> DynamicImage {
        DynamicImage::new(self.width, self.height, image::ColorType::Rgb8)
    }
}

#[derive(Debug)]
pub struct BirdSimulation{
    pub params: BirdSimulationParameters

}

impl Default for BirdSimulation {
    fn default() -> Self {
        BirdSimulation {
            params: BirdSimulationParameters::default()
        }
    }    
}

impl Simulation for BirdSimulation {
    fn set_parameters(&mut self, params: SimulationParameters) {
        self.params.from_parameters(params);
    }

    fn run(&self) -> Result<(), String> {
        let _ = &self.params.run();
        Ok(())
    }

    fn parameter_descriptions(&self) -> Vec<ParameterDescription> {
        BirdSimulationParameters::parameter_descriptions()
    }

    fn get_name(&self) -> String {
        "Birds".to_string()
    }
}
