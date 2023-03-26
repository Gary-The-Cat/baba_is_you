use crate::data_structures::enums::nodes::*;

pub enum Node{
    Noun(noun::Noun),
    Property(properties::Property),
    Operator(operators::Operator),
}