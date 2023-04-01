use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Operator{
    Is,
    And,
    Or,
}