mod ecs;
mod data_structures;
use bevy::prelude::*;
use ecs::systems::index_debugger;
use ecs::systems::index_position_updater;
use data_structures::enums::nodes::noun::*;
use  data_structures::enums::node::Node;

fn main() {
    let test_node: Node = Node::Noun(Noun::Baba);

    match test_node {
        Node::Noun(noun) => {},
        Node::Operator(operator) => {},
        Node::Property(property) => {},
    };

    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
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