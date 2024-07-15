use bevy::{app::Plugin, prelude::Resource};
use noise::{NoiseFn, OpenSimplex, RidgedMulti};

use crate::local_world::{x_from_coord, z_from_coord};




pub struct RandomPlugin;
impl Plugin for RandomPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(HeightmapNoise(RidgedMulti::new(14068690)));
    }
}


#[derive(Resource)]
pub struct HeightmapNoise(pub RidgedMulti<OpenSimplex>);

impl HeightmapNoise{
    pub fn height_at_xz(&self, x: f32, z: f32) -> f32 {
        self.0.get([x as f64 / 10.0 + 0.46721, z as f64 / 10.0 + 0.46721]) as f32
    }

    pub fn height_at_coord(&self, i: i32, j: i32) -> f32 {
        self.height_at_xz(x_from_coord(i, j), z_from_coord(i, j))
    }
}