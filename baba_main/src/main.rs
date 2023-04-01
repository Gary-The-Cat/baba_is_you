mod ecs;
mod data_structures;
use bevy::prelude::*;
use data_structures::grid::Grid;
use data_structures::grid_configuration::GridConfiguration;
use ecs::systems::index_debugger;
use ecs::systems::index_position_updater;

fn main() {
    let grid = Grid {
        configuration: GridConfiguration{
            index_size_x: 20,
            index_size_y: 20,
            cell_size_x: 20.0,
            cell_size_y: 20.0,
        },
    };

    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .insert_resource(grid)
    .add_system(index_position_updater::update)
    .add_system(index_debugger::index_debugger_print)
    .run();
}

pub fn setup(mut commands : Commands){
    commands.spawn((
        ecs::components::position_index::PositionIndex{x: 2, y: 2 },
        ecs::components::position::Position{x: 2.0, y: 2.0 }
    ));
}