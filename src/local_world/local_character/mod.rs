use bevy::{app::{Plugin, PreStartup, Startup, Update}, asset::{Assets, Handle}, color::Color, math::vec3, pbr::{MaterialMeshBundle, StandardMaterial}, prelude::{Capsule3d, Commands, Component, EventReader, Query, Res, ResMut, Resource, With}, render::mesh::Mesh, transform::components::Transform};
use noise::{NoiseFn, OpenSimplex, RidgedMulti};

use super::{hex_tile::FRAC_1_SQRT_3, PlayerMovedEvent};



pub struct LocalCharacterPlugin;
impl Plugin for LocalCharacterPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(PreStartup, create_handles)
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_character);
    }
}



#[derive(Resource)]
pub struct CharacterHandles{
    pub char_mesh: Handle<Mesh>
}

#[derive(Component)]
pub struct CharacterMarker;


fn create_handles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>
) {
    commands.insert_resource(CharacterHandles{
        char_mesh: meshes.add(Capsule3d::new(0.2, 0.3)),
    });
}

fn spawn_player(
    char_mesh: Res<CharacterHandles>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //todo: this is bad, should go into a resource or component or smth (duped from the terrain height gen code)
    let noise: RidgedMulti<OpenSimplex> = RidgedMulti::new(14068690);
    let x_pos = 0.0;
    let z_pos = 0.0;
    let y_pos = noise.get([0.46721, 0.46721]) as f32 + 0.3;

    commands.spawn((MaterialMeshBundle{
        mesh: char_mesh.char_mesh.clone(),
        material: materials.add(StandardMaterial::from(Color::linear_rgba(1.0, 1.0, 0.0, 1.0))),
        transform: Transform::from_translation(vec3(x_pos, y_pos, z_pos)),
        ..Default::default()
    },
    CharacterMarker
    ));
}


fn move_character(
    mut reader: EventReader<PlayerMovedEvent>,
    mut char_q: Query<&mut Transform, With<CharacterMarker>>
){
    let mut char_pos = char_q.single_mut();
    let noise: RidgedMulti<OpenSimplex> = RidgedMulti::new(14068690);
    for event in reader.read(){
        let x_pos = event.to.0 as f32 * FRAC_1_SQRT_3 * 1.5;
        let z_pos = event.to.1 as f32 + if event.to.0 % 2 != 0 {0.5} else {0.0};
        let y_pos = noise.get([event.to.0 as f64 / 10.0 + 0.46721, event.to.1 as f64 / 10.0 + 0.46721]) as f32 + 0.3;
        char_pos.translation = vec3(x_pos, y_pos, z_pos);
    }

}