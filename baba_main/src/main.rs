mod application;

use backend::data_structures::level_data::LevelData;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use backend::data_structures::grid::Grid;
use backend::data_structures::grid_configuration::GridConfiguration;
use backend::data_structures::enums::node::Node;
use bevy::window::PrimaryWindow;
use backend::ecs::components::node_type::NodeType;
use backend::ecs::systems::index_debugger;
use backend::ecs::systems::index_position_updater;
use backend::ecs::components::position_index::PositionIndex;

fn main() {
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
    .add_startup_system(setup_camera)
    .add_system(set_grid)
    .add_system(movement_test)
    .add_system(index_movement_test)
    .add_system(index_position_updater::update)
    .add_system(index_debugger::index_debugger_print).run();
}

pub fn setup_camera(
    mut commands : Commands,
    window_query: Query<&Window, With<PrimaryWindow>>){

    let window = window_query.get_single().unwrap();
    let x = window.width() / 2.0;
    let y = window.height() / 2.0;

    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(x, y, 0.0),
        camera_2d: Camera2d { clear_color: ClearColorConfig::Custom(Color::rgb(0.8, 0.4, 0.2)) },
        ..Default::default()
    });
}

pub fn index_movement_test(
    mut query: Query<(&PositionIndex, &mut Transform)>,
    grid: Res<Grid>){
    for (position_index, mut transform) in query.iter_mut(){
        let position = grid.cell_center_from_index((position_index.x, position_index.y)).unwrap();
        transform.translation.x = position.0;
        transform.translation.y = position.1;
    }
}

pub fn movement_test(
    mut query: Query<&mut PositionIndex>,
    keyboard_input: Res<Input<KeyCode>>){
        if keyboard_input.just_pressed(KeyCode::W){
            for mut index_position in query.iter_mut(){
                index_position.y = index_position.y + 1;
            }
        }
    }

pub fn set_grid(
    mut commands : Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    existing_node_query: Query<Entity, With<NodeType>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>){

    if keyboard_input.just_pressed(KeyCode::L){

        // Remove all existing nodes from the world.
        for entity in existing_node_query.iter(){
            commands.entity(entity).despawn();
        }

        let level_path = application::file_system::get_level_path("basic.json".to_string());
        let lines = std::fs::read_to_string(level_path).unwrap();

        let level_data: LevelData = serde_json::from_str(&lines).unwrap();

        // Get the half window size to create the grid.
        let window = window_query.get_single().unwrap();
        let grid = Grid{
            configuration: level_data.grid,
            centre: (window.width() / 2.0, window.height() / 2.0),
        };

        // Spawn the grid background
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(grid.width(), grid.height())),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(grid.centre.0, grid.centre.1, 0.)),
            ..default()
        });

        // Create all node entities that are in the level.
        for node_visual in level_data.nodes{

            // Load asset for node:
            let asset_name = backend::data_structures::enums::node_visual_map::node_to_visual(node_visual.node);
            let asset_path = application::file_system::get_asset_path(asset_name);
            let asset = asset_server.load(asset_path);

            let grid_position = grid.cell_center_from_index(node_visual.index_position).unwrap();

            let mut entity_commands = commands.spawn((
                backend::ecs::components::position_index::PositionIndex{ x: node_visual.index_position.0, y: node_visual.index_position.1 },
                backend::ecs::components::position::Position{ ..default() },
                backend::ecs::components::node_type::NodeType{ node: node_visual.node },
                SpriteBundle{
                    transform: Transform::from_xyz(grid_position.0, grid_position.1, 0.0),
                    texture: asset,
                    ..default()
                }
            ));

            // Give all text nodes non-transient push component
            match node_visual.node {
                Node::Object(_) => {},
                Node::Noun(_) => { entity_commands.insert(backend::ecs::components::push::Push{ is_transient: false }); },
                Node::Operator(_) => { entity_commands.insert(backend::ecs::components::push::Push{ is_transient: false }); },
                Node::Property(_) => { entity_commands.insert(backend::ecs::components::push::Push{ is_transient: false }); },
            }
        }
        
        // Overwrites the existing grid resource with the new one for this level.
        commands.insert_resource(grid);
    }
}