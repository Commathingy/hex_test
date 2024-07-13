mod hex_tile;
mod local_camera;
mod local_character;

use bevy::{app::{Plugin, Update}, asset::{Assets, Handle}, color::Color, ecs::{event::{Event, EventReader}, system::{Commands, Query, Res}}, hierarchy::Children, pbr::StandardMaterial, prelude::{Changed, IntoSystemConfigs}};

use local_camera::LocalCameraPlugin;
use local_character::LocalCharacterPlugin;
use hex_tile::HexPlugin;

use crate::graph_functions;

use self::hex_tile::{ColourTransition, HexPositionMap, TileExploredState};


pub use hex_tile::HexTile; ///////////////////////////


pub struct LocalWorldPlugin;
impl Plugin for LocalWorldPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_event::<PlayerMovedEvent>()
        .add_plugins(HexPlugin)
        .add_plugins(LocalCameraPlugin)
        .add_plugins(LocalCharacterPlugin)
        .add_systems(Update, (update_tile_states, change_tile_colours.after(update_tile_states)));
    }
}

// ============================
// Systems
// ============================

pub fn update_tile_states(
    tiles_map: Res<HexPositionMap>,
    mut reader: EventReader<PlayerMovedEvent>,
    mut tiles: Query<&mut HexTile>
) {
    for event in reader.read(){
        let start_ent = *tiles_map.map.get(&event.from).unwrap();
        let end_ent = *tiles_map.map.get(&event.to).unwrap();
        let left = graph_functions::within_steps(start_ent, 3, &tiles.to_readonly()).unwrap();
        let entered = graph_functions::within_steps(end_ent, 3, &tiles.to_readonly()).unwrap();

        for (ent, _) in left.into_iter(){
            tiles.get_mut(ent).unwrap().explored_state = TileExploredState::Explored;
        }
        for (ent, _) in entered.into_iter(){
            tiles.get_mut(ent).unwrap().explored_state = TileExploredState::Visible;
        }
    }
}



fn change_tile_colours(
    mut commands: Commands,
    col_parent: Query<(&HexTile, &Children), Changed<HexTile>>,
    colours: Query<&Handle<StandardMaterial>>,
    mats: Res<Assets<StandardMaterial>>,
) {
    //get the current colour of the tile and add transition
    for (tile, children) in col_parent.iter(){
        let new_colour = match tile.explored_state{
            TileExploredState::Hidden => Color::linear_rgba(0.0, 0.0, 0.0, 1.0),
            TileExploredState::Explored => Color::linear_rgba(0.5, 0.5, 0.5, 1.0),
            TileExploredState::Visible => Color::linear_rgba(0.0, 1.0, 0.0, 1.0),
        };
        for &child in children{
            if let Ok(handle) = colours.get(child){
                let current_col = mats.get(handle).unwrap().base_color;
                commands.entity(child).insert(ColourTransition::new(current_col, new_colour, 2.0));
            }
        }
    }
    
}



#[derive(Event)]
pub struct PlayerMovedEvent{
    pub from: (i32, i32),
    pub to: (i32, i32)
}