pub use crate::parameters::*;
pub use crate::julia_sets_simulation::JuliaSetSimulation;
pub use crate::birds::BirdSimulation;

pub trait Simulation {
    fn run(&self) -> Result<(), String>;
    fn set_parameters(&self, params: SimulationParameters);
    fn get_name(&self) -> String;
    fn parameter_descriptions(&self) -> Vec<ParameterDescription>;
}

pub async fn simulate(simulation_type: String, params: SimulationParameters) -> Option<String> {
    let simulation_type: String = simulation_type;
    let simulation: Box<dyn Simulation>;

    simulation = match simulation_type.as_str() {
        "birds" => Box::new(BirdSimulation{}),
        _ => Box::new(JuliaSetSimulation{})
    };
    simulation.set_parameters(params);
    simulation.run();

}
