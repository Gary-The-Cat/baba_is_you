use bevy::prelude::*;
use crate::data_structures::enums::node::Node;

#[derive(Component)]
pub struct NodeType{
    pub node: Node
}