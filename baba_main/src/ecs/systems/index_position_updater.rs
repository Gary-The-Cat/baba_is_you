use bevy::prelude::*;
use crate::data_structures::grid::Grid;
use crate::data_structures::grid_configuration::GridConfiguration;
use crate::ecs::components::position::Position;
use crate::ecs::components::position_index::PositionIndex;

static GRID: Grid = Grid {
    configuration: GridConfiguration{
        index_size_x: 20,
        index_size_y: 20,
        cell_size_x: 20.0,
        cell_size_y: 20.0,
    },
};

pub fn update(mut query: Query<(&mut Position, &PositionIndex)>){

    for (mut position, position_index) in query.iter_mut(){
        let grid_position = GRID.cell_top_left_from_index((position_index.x, position_index.y));
        match  grid_position {
            None => {},
            Some((x, y)) =>{
                position.x = x;
                position.y = y;
            } ,
        }
    }
}