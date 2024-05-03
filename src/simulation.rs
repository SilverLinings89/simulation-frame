pub use crate::birds::BirdSimulation;
pub use crate::julia_sets_simulation::JuliaSetSimulation;
pub use crate::parameters::*;

pub trait Simulation {
    fn run(&self) -> Result<(), String>;
    fn set_parameters(&mut self, params: SimulationParameters);
    fn get_name(&self) -> String;
    fn parameter_descriptions(&self) -> Vec<ParameterDescription>;
    fn get_simulation_details(&self) -> SimulationDetails {
        SimulationDetails {
            name: self.get_name(),
            parameters: self.parameter_descriptions(),
        }
    }
}

pub async fn simulate(simulation_type: String, params: SimulationParameters) -> Option<String> {
    let simulation_type: String = simulation_type;
    let mut simulation: Box<dyn Simulation>;

    simulation = match simulation_type.as_str() {
        "birds" => Box::new(BirdSimulation {
            ..Default::default()
        }),
        _ => Box::new(JuliaSetSimulation {
            ..Default::default()
        }),
    };
    simulation.set_parameters(params);
    let _ = simulation.run();
    Some("Simulation completed".to_string())
}
