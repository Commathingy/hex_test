mod hex_tile;

use bevy::{app::{Plugin, Update}, render::color::Color, ecs::{system::{Commands, Query, Res}, query::With, event::{EventReader, Event}}, hierarchy::Children, asset::{Handle, Assets}, pbr::StandardMaterial};

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
        .add_systems(Update, update_tile_states);
    }
}

// ============================
// Systems
// ============================

pub fn update_tile_states(
    mut commands: Commands,
    col_parent: Query<&Children, With<HexTile>>,
    colours: Query<&Handle<StandardMaterial>>,
    mats: Res<Assets<StandardMaterial>>,
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

            //get the current colour of the tile and add transition
            let col_ent = col_parent.get(ent).unwrap();
            for &child in col_ent{
                if let Ok(handle) = colours.get(child){
                    let current_col = mats.get(handle).unwrap().base_color;
                    commands.entity(child).insert(ColourTransition::new(current_col, Color::GRAY, 2.0));
                }
            }
        }
        for (ent, _) in entered.into_iter(){
            tiles.get_mut(ent).unwrap().explored_state = TileExploredState::Visible;

            //get the current colour of the tile and add transition
            let col_ent = col_parent.get(ent).unwrap();
            for &child in col_ent{
                if let Ok(handle) = colours.get(child){
                    let current_col = mats.get(handle).unwrap().base_color;
                    commands.entity(child).insert(ColourTransition::new(current_col, Color::GREEN, 2.0));
                }
            }
        }
    }
}


#[derive(Event)]
pub struct PlayerMovedEvent{
    pub from: (i32, i32),
    pub to: (i32, i32)
}