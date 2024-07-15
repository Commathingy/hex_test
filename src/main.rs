mod local_world;
mod graph_functions;
mod player;
mod random_gens;

use bevy::prelude::*;
use bevy_mod_raycast::deferred::{RaycastSource, DeferredRaycastingPlugin};
use local_world::{HexTile, LocalWorldPlugin, PlayerMovedEvent};
use random_gens::RandomPlugin;



//todo:
//add sparse set storage for certain components -> anything used for animations ig
//animation components are never removed
//for rotatingw component continuously- one frame gap due to despawn/ spawning -> adjust order of ticking etc 


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(DeferredRaycastingPlugin::<()>::default())
        .add_plugins(RandomPlugin)
        .add_plugins(LocalWorldPlugin)
        .add_systems(Update, test_move)
        .run();
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState{
    #[default]
    MainMenu,
    LocalWorld,
}


fn test_move(
    mut state: ResMut<NextState<GameState>>,
    mut current_pos: Local<(i32, i32)>,
    input: Res<ButtonInput<MouseButton>>,
    mut writer: EventWriter<PlayerMovedEvent>,
    raycast: Query<&RaycastSource<()>>,
    meshes: Query<&Parent, With<Handle<Mesh>>>,
    hexes: Query<&HexTile>
) {

    if input.just_pressed(MouseButton::Right){
        state.set(GameState::LocalWorld);
    }
    
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





