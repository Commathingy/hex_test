use bevy::{app::{Plugin, Update}, prelude::{IntoSystemConfigs, Query, Res, With}, state::state::OnEnter, transform::components::Transform};

use crate::{random_gens::HeightmapNoise, GameState};

use super::{HexSpawnSet, HexTile};


pub struct TerrainPlugin;

impl Plugin for TerrainPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::LocalWorld), add_elevation.after(HexSpawnSet));
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

) {

}