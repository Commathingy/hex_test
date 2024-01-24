mod local_world;
mod graph_functions;
mod camera;

use bevy::{
    prelude::*,
    ecs::system::EntityCommand, 
};
use bevy_mod_raycast::deferred::{RaycastSource, DeferredRaycastingPlugin};
use camera::CameraPlugin;
use local_world::{LocalWorldPlugin, PlayerMovedEvent, HexTile};


//todo:
//add sparse set storage for certain components -> anything used for animations ig
//animation components are never removed
//for rotating component continuously- one frame gap due to despawn/ spawning -> adjust order of ticking etc 


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DeferredRaycastingPlugin::<()>::default())
        .add_plugins(LocalWorldPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Update, test_move)
        .run();
}







struct AddIfMissing<T: Component>{
    component: T,
}
impl<T: Component> EntityCommand for AddIfMissing<T>{
    fn apply(self, id: Entity, world: &mut World) {
        match world.get_entity_mut(id){
            Some(mut ent_ref) => {
                if !ent_ref.contains::<T>() {
                    ent_ref.insert(self.component);
                } else {
                    return;
                }
            },
            None => return,
        }
    }
}







fn test_move(
    mut current_pos: Local<(i32, i32)>,
    input: Res<Input<MouseButton>>,
    mut writer: EventWriter<PlayerMovedEvent>,
    raycast: Query<&RaycastSource<()>>,
    meshes: Query<&Parent, With<Handle<Mesh>>>,
    hexes: Query<&HexTile>
) {
    
    if input.just_pressed(MouseButton::Left){
        if let Some((ent, _)) = raycast.single().get_nearest_intersection(){
            let new_pos = hexes.get(meshes.get(ent).unwrap().get()).unwrap().position;
            writer.send(PlayerMovedEvent {
                from: *current_pos, 
                to: new_pos
            });
            *current_pos = new_pos;
        }
        
    }
}





