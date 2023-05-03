use bevy::prelude::*;
use crate::ecs::components::push::Push;

pub fn RemoveAllTransientComponents(
    world: &mut World,
    commands: &mut Commands){

    // // for (entity, push) in push_query.iter(){
    // //     if push.is_transient{
    // //         commands.entity(entity).remove::<Push>();
    // //     }
    // // }
}
