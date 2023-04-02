use crate::data_structures::enums::nodes::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Node{
    Object(object::Object),
    Noun(noun::Noun),
    Property(properties::Property),
    Operator(operators::Operator),
}