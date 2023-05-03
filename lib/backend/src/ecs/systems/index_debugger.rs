use bevy::prelude::*;
use crate::ecs::components::position_index::PositionIndex;

pub fn index_debugger_print(query: Query<&PositionIndex>){
    for _item in query.iter(){
        // println!("X: {} Y: {}", item.x, item.y)
    }
}   