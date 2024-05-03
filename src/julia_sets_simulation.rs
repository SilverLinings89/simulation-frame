use serde::{Serialize, Deserialize};
use image::{DynamicImage, GenericImage, Rgba};

pub use crate::complex::Complex;
pub use crate::simulation::*;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 800;
const DEFAULT_C_REAL: f64 = 0.0;
const DEFAULT_C_IMAGINARY: f64 = 0.0;
const DEFAULT_MAX_ITER: u32 = 1000;
const DEFAULT_ESCAPE_RADIUS: f64 = 2.0;


#[derive(Debug, Serialize, Deserialize)]
pub struct JuliaSetParameters {
    pub width: u32,
    pub height: u32,
    pub c_real: f64,
    pub c_imaginary: f64,
    pub max_iter: u32,
    pub escape_radius: f64,

}

impl std::default::Default for JuliaSetParameters {
    fn default() -> Self {
        JuliaSetParameters {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            c_real: DEFAULT_C_REAL,
            c_imaginary: DEFAULT_C_IMAGINARY,
            max_iter: DEFAULT_MAX_ITER,
            escape_radius: DEFAULT_ESCAPE_RADIUS,
        }
    }
}

impl JuliaSetParameters {
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
                name: "c_real".to_string(),
                description: "Real part of the constant c".to_string(),
                data_type: "integer".to_string(),
                default: ParameterType::Float(DEFAULT_C_REAL),
                required: true,
                lower_bound: None,
                upper_bound: None,
                step: None
            },
            ParameterDescription {
                name: "c_imaginary".to_string(),
                description: "Imaginary part of the constant c".to_string(),
                data_type: "integer".to_string(),
                default: ParameterType::Float(DEFAULT_C_IMAGINARY),
                required: true,
                lower_bound: None,
                upper_bound: None,
                step: None
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
                name: "escape_radius".to_string(),
                description: "Escape radius".to_string(),
                data_type: "integer".to_string(),
                default: ParameterType::Float(DEFAULT_ESCAPE_RADIUS),
                required: true,
                lower_bound: Some(ParameterType::Integer(1)),
                upper_bound: None,
                step: None
            },
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
                "c_real" => {
                    if let ParameterType::Float(value) = p.value {
                        self.c_real = value;
                    }
                }
                "c_imaginary" => {
                    if let ParameterType::Float(value) = p.value {
                        self.c_imaginary = value;
                    }
                }
                "max_iter" => {
                    if let ParameterType::UnsignedInteger(value) = p.value {
                        self.max_iter = value;
                    }
                }
                "escape_radius" => {
                    if let ParameterType::Float(value) = p.value {
                        self.escape_radius = value;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn run(&self) -> DynamicImage {
        let mut img = DynamicImage::new(self.width, self.height, image::ColorType::Rgb8);

        for y in 0..self.height {
            for x in 0..self.width {
                let zx = 3.0 * (x as f64 - self.width as f64 / 2.0) / (self.width as f64);
                let zy = 2.0 * (y as f64 - self.height as f64 / 2.0) / (self.height as f64);
                let mut z = Complex::new(zx, zy);

                let mut i = 0;
                while i < self.max_iter && z.norm() <= self.escape_radius {
                    z = z.square().add(Complex::new(self.c_real, self.c_imaginary));
                    i += 1;
                }

                let color: Rgba<u8> = if i == self.max_iter {
                    Rgba([0, 0, 0,0])
                } else {
                    let ratio = (i as f64) / (self.max_iter as f64);
                    let r = (255.0 * ratio) as u8;
                    let g = (255.0 * (1.0 - ratio)) as u8;
                    Rgba([r, g, 0, 255])
                };

                img.put_pixel(x, y, color);
            }
        }

        img
    }
}

#[derive(Debug)]
pub struct JuliaSetSimulation{
    pub params: JuliaSetParameters
}

impl Default for JuliaSetSimulation {
    fn default() -> Self {
        JuliaSetSimulation {
            params: JuliaSetParameters::default()
        }
    }
}

impl Simulation for JuliaSetSimulation {
    fn set_parameters(&mut self, params: SimulationParameters) {
        self.params.from_parameters(params);
    }

    fn run(&self) -> Result<(), String> {
        let _ = &self.params.run();
        Ok(())
    }

    fn parameter_descriptions(&self) -> Vec<ParameterDescription> {
        JuliaSetParameters::parameter_descriptions()
    }

    fn get_name(&self) -> String {
        "JuliaSets".to_string()
    }
}
