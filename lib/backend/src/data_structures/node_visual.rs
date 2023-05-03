use crate::data_structures::enums::node::Node;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NodeVisual{
    pub index_position: (u32, u32),
    pub node: Node,
}