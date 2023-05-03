use bevy::prelude::*;
use crate::data_structures::grid::Grid;
use crate::ecs::components::position::Position;
use crate::ecs::components::position_index::PositionIndex;

pub fn update(
    mut query: Query<(&mut Position, &PositionIndex)>,
    grid: Res<Grid>){

    for (mut position, position_index) in query.iter_mut(){
        let grid_position = grid.cell_top_left_from_index((position_index.x, position_index.y));
        match  grid_position {
            None => {},
            Some((x, y)) =>{
                position.x = x;
                position.y = y;
            } ,
        }
    }
}