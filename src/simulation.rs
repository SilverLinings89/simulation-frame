pub use crate::parameters::*;
pub use crate::julia_sets_simulation::JuliaSetSimulation;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum Simulation {
   Birds(JuliaSetSimulation),
   Shooter(JuliaSetSimulation),
   JuliaSets(JuliaSetSimulation)
}


pub trait Simulate {
    type Parameters;

    fn run(&self) -> Result<(), String>;
    
    fn parameter_descriptions() -> Vec<ParameterDescription>;

    fn new(params: SimulationParameters) -> Self ;

    fn get_name() -> String;
}


impl Simulation {
    pub fn run(&self) -> Result<(), String> {
        unimplemented!()
    }

    pub fn get_name(&self) -> String {
        self.get_name()
    }
}


pub async fn simulate(simulation_type: String, params: SimulationParameters) -> Option<String> {
    let simulation_type: String = simulation_type;
    let simulation: Option<Simulation> = match simulation_type.as_str() {
        "Birds" => Some(Simulation::Birds(JuliaSetSimulation::new(params))),
        "Shooter" => Some(Simulation::Shooter(JuliaSetSimulation::new(params))),
        "JuliaSets" => Some(Simulation::JuliaSets(JuliaSetSimulation::new(params))),
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