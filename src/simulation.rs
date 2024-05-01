pub use crate::parameters::*;
pub use crate::julia_sets_simulation::JuliaSetSimulation;
pub use crate::birds::BirdSimulation;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum Simulation {
   JuliaSet(JuliaSetSimulation),
   Bird(BirdSimulation)
}


pub trait Simulate {
    type Parameters;

    fn run(&self) -> Result<(), String>;
    
    fn parameter_descriptions() -> Vec<ParameterDescription>;

    fn new(params: SimulationParameters) -> Self;

    fn get_name(&self) -> String;
}

impl Simulation {
    pub fn run(&self) -> Result<(), String> {
        unimplemented!()
    }
    
    pub fn get_name(&self) -> String {
        match self {
            Simulation::JuliaSet(s) => "Julia".to_string(),
            Simulation::Bird(s) => "Bird".to_string(),
        }
    }
}

pub async fn simulate(simulation_type: String, params: SimulationParameters) -> Option<String> {
    let simulation_type: String = simulation_type;
    let simulation: Option<Simulation> = match simulation_type.as_str() {
        "JuliaSets" => Some(Simulation::JuliaSet(JuliaSetSimulation::new(params))),
        _ => None
    };

    match simulation {
        Some(_) =>  {
            let result = simulation?.run();
            match result {
                Ok(_) => Some("Simulation ran successfully".to_string()),
                Err(e) => Some(e)
            }
        },
        None => None
    }
}
