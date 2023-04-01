use bevy::prelude::*;

pub fn index_debugger_print(query: Query<&super::super::components::position_index::PositionIndex>){
    for item in query.iter(){
        // println!("X: {} Y: {}", item.x, item.y)
    }
}