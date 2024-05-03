mod utils;

use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
mod birds;
mod complex;
mod julia_sets_simulation;
mod parameters;
mod simulation;

use birds::BirdSimulation;
use julia_sets_simulation::JuliaSetSimulation;
use parameters::SimulationDetails;
use simulation::Simulation;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn start() -> JsValue {
    let simulations: Vec<Box<dyn Simulation>> = vec![
        Box::new(BirdSimulation::default()),
        Box::new(JuliaSetSimulation::default()),
    ];
    let mut ret: Vec<SimulationDetails> = vec![];
    for s in simulations {
        ret.push(s.get_simulation_details());
    }
    serde_wasm_bindgen::to_value(&ret).unwrap()
}
