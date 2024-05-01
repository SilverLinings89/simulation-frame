use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ParameterType {
    UnsignedInteger(u32),
    Integer(i32),
    Float(f64),
    Boolean(bool),
    String(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub value: ParameterType,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterDescription {
    pub name: String,
    pub description: String,
    pub default: ParameterType,
    pub required: bool,
    pub data_type: String, 
    pub lower_bound: Option<ParameterType>,
    pub upper_bound: Option<ParameterType>,
    pub step: Option<ParameterType>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationParameters {
    pub params: Vec<Parameter>,
}
