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

impl ToString for ParameterDescription {
    fn to_string(&self) -> String {
        match self.lower_bound {
            Some(_) => format!(
                "{}({}): {}. Between {:?} and {:?} in steps of {:?}.",
                self.name,
                self.data_type,
                self.description,
                self.lower_bound,
                self.upper_bound,
                self.step
            ),
            None => format!("{}({}): {}.", self.name, self.data_type, self.description),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationParameters {
    pub params: Vec<Parameter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationDetails {
    pub name: String,
    pub parameters: Vec<ParameterDescription>,
}
