mod utils;

use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;

mod simulation; 
mod parameters;
mod julia_sets_simulation;
mod complex;
mod birds;

use simulation::Simulation;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    println!("TEST");
    let sims =get_simulations();
    println!("TEST2");
    let message = format!("Hello, simulation-frame! sims are {:?}", sims);
    println!("TEST3");
    message.as_str();
}

pub fn get_simulations() -> Vec<String>{
    Simulation::iter().map(|s| s.get_name()).collect()
}

