use bevy::{app::{Plugin, Update}, asset::{Assets, Handle}, color::Color, hierarchy::Children, pbr::StandardMaterial, prelude::{IntoSystemConfigs, Query, Res, ResMut, With}, state::state::OnEnter, transform::components::Transform};

use crate::{random_gens::HeightmapNoise, GameState};

use super::{hex_neighbours, HexSpawnSet, HexTile};


pub struct TerrainPlugin;

impl Plugin for TerrainPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::LocalWorld), (add_elevation, determine_biomes).after(HexSpawnSet));
    }
}




fn add_elevation(
    mut tiles_q: Query<&mut Transform, With<HexTile>>,
    heightmap_noise: Res<HeightmapNoise>
){
    for mut transform in tiles_q.iter_mut(){
        transform.translation.y = heightmap_noise.height_at_xz(transform.translation.x, transform.translation.z);
    }
}

fn determine_biomes(
    tiles_q: Query<(&HexTile, &Children)>,
    colours: Query<&Handle<StandardMaterial>>,
    heightmap_noise: Res<HeightmapNoise>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (tile, children) in tiles_q.iter(){
        let self_height = heightmap_noise.height_at_coord(tile.position.0, tile.position.1);
        let mut higher_than = 0;

        for other_pos in hex_neighbours(&tile.position){
            higher_than += if self_height >= heightmap_noise.height_at_coord(other_pos.0, other_pos.1) {1} else {0};
        }

        //iterate over the children, filter-mapping to the assosciated Handle<StandardMaterial>
        //there should be exactly one of these, but for robustness we flatmap to convert into the material
        let mat = match children.iter()
        .filter_map(|child|{colours.get(*child).ok()})
        .next().and_then(|handle| materials.get_mut(handle)){
            Some(material) => material,
            None => continue,
        };

        mat.base_color = match self_height {
            height if height > 0.5 && higher_than > 3 => Color::linear_rgba(0.3, 0.3, 0.3, 1.0),
            height if height > 0.5 => Color::linear_rgba(0.5, 0.5, 0.5, 1.0),
            _ => Color::linear_rgba(1.0, 0.0, 1.0, 1.0)
        }.into();
        
        
    }
}