use crate::data_structures::enums::node::Node;
use crate::data_structures::enums::nodes::noun::Noun;
use crate::data_structures::enums::nodes::object::Object;
use crate::data_structures::enums::nodes::properties::Property;
use crate::data_structures::enums::nodes::operators::Operator;

pub fn node_to_visual(node: Node) -> String{
    return match node{
        Node::Object(object) => match object{
            Object::Baba => "node_images\\objects\\Baba.png".to_string(),
            Object::Flag => "node_images\\objects\\Flag.png".to_string(),
            Object::Key => "node_images\\objects\\Key.png".to_string(),
            Object::Rock => "node_images\\objects\\Rock.png".to_string(),
            Object::Wall => "node_images\\objects\\Wall.png".to_string(),
        }
        Node::Noun(noun) => match noun{
            Noun::Baba => "node_images\\text\\objects\\Baba.png".to_string(),
            Noun::Flag => "node_images\\text\\objects\\Flag.png".to_string(),
            Noun::Key => "node_images\\text\\objects\\Key.png".to_string(),
            Noun::Rock => "node_images\\text\\objects\\Rock.png".to_string(),
            Noun::Wall => "node_images\\text\\objects\\Wall.png".to_string(),
        }
        Node::Property(property) => match property {
            Property::Defeat => "node_images\\text\\properties\\Defeat.png".to_string(),
            Property::Hot => "node_images\\text\\properties\\Hot.png".to_string(),
            Property::Melt => "node_images\\text\\properties\\Melt.png".to_string(),
            Property::Open => "node_images\\text\\properties\\Open.png".to_string(),
            Property::Push => "node_images\\text\\properties\\Push.png".to_string(),
            Property::Shut => "node_images\\text\\properties\\Shut.png".to_string(),
            Property::Sink => "node_images\\text\\properties\\Sink.png".to_string(),
            Property::Stop => "node_images\\text\\properties\\Stop.png".to_string(),
            Property::Tele => "node_images\\text\\properties\\Tele.png".to_string(),
            Property::Win => "node_images\\text\\properties\\Win.png".to_string(),
            Property::You => "node_images\\text\\properties\\You.png".to_string(),
        }
        Node::Operator(operator) => match operator {
            Operator::Is => "node_images\\text\\operators\\Is.png".to_string(),
            Operator::And => "node_images\\text\\operators\\And.png".to_string(),
            Operator::On => "node_images\\text\\operators\\On.png".to_string(),
        }
    }
}