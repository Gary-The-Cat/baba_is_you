mod application;
mod ecs;
mod data_structures;
use bevy::prelude::*;
use data_structures::grid::Grid;
use data_structures::grid_configuration::GridConfiguration;
use bevy::window::PrimaryWindow;
use ecs::systems::index_debugger;
use ecs::systems::index_position_updater;

// Testing
use data_structures::enums::node::Node;
use data_structures::enums::nodes::object::Object;

fn main() {

    // Enum serialization test
    let node: Node = Node::Object(Object::Baba);
    let serialized = serde_json::to_string(&node).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();

    // Temporary hack, needed because the systems will start straight away before a level has been loaded and the grid resource
    // is needed by the update system.
    let grid = Grid{
        configuration: GridConfiguration{
            index_size_x: 20,
            index_size_y: 20,
            cell_size_x: 20.0,
            cell_size_y: 20.0,
        },
        centre: (0.0, 0.0),
    };

    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(grid)
    .add_startup_system(setup)
    .add_startup_system(setup_camera)
    .add_system(set_grid)
    .add_system(index_position_updater::update)
    .add_system(index_debugger::index_debugger_print).run();
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

pub fn set_grid(
    mut commands : Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>){

    if keyboard_input.just_pressed(KeyCode::L){

        let window = window_query.get_single().unwrap();
        let x = window.width() / 2.0;
        let y = window.height() / 2.0;

        let configuration = GridConfiguration{
            index_size_x: 20,
            index_size_y: 20,
            cell_size_x: 20.0,
            cell_size_y: 20.0,
        };

        let grid = Grid{
            configuration: configuration,
            centre: (x, y),
        };

        // I think that this should be init? we would need to impl default for grid.
        commands.insert_resource(grid);

        let lines = application::file_system::read_lines("C:/dev/BabaLevels/level.csv".to_string());

        let level_data = data_structures::level_data::LevelData::create_from_lines(lines);

        println!("Someone pressed the L key.");
    }
}