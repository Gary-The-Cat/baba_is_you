mod ecs;
mod data_structures;
use bevy::prelude::*;
use data_structures::grid::Grid;
use data_structures::grid_configuration::GridConfiguration;
use bevy::window::PrimaryWindow;
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
    .insert_resource(grid)
    .add_startup_system(setup)
    .add_startup_system(setup_camera)
    .add_system(index_position_updater::update)
    .add_system(index_debugger::index_debugger_print)
    .run();
}

pub fn setup(
    mut commands : Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>){

    let window = window_query.get_single().unwrap();
    let x = window.width() / 2.0;
    let y = window.height() / 2.0;

    commands.spawn((
        ecs::components::position_index::PositionIndex{ x: 2, y: 2 },
        ecs::components::position::Position{ x: 2.0, y: 2.0 }
    ));

    let asset = asset_server.load("BabaIsYouIcon.png");

    commands.spawn(SpriteBundle{
        transform: Transform::from_xyz(x, y, 0.0),
        texture: asset,
        ..Default::default()
    });
}

pub fn setup_camera(
    mut commands : Commands,
    window_query: Query<&Window, With<PrimaryWindow>>){

    let window = window_query.get_single().unwrap();
    let x = window.width() / 2.0;
    let y = window.height() / 2.0;

    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(x, y, 0.0),
        ..Default::default()
    });
}