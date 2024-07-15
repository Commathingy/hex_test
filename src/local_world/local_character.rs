use bevy::{app::{Plugin, PreStartup, Startup, Update}, asset::{Assets, Handle}, color::Color, math::vec3, pbr::{MaterialMeshBundle, StandardMaterial}, prelude::{Capsule3d, Commands, Component, EventReader, Query, Res, ResMut, Resource, With}, render::mesh::Mesh, transform::components::Transform};

use crate::random_gens::HeightmapNoise;

use super::{x_from_coord, z_from_coord, PlayerMovedEvent};



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
    height_noise: Res<HeightmapNoise>
) {
    let x_pos = 0.0;
    let z_pos = 0.0;
    let y_pos = height_noise.height_at_xz(0.0, 0.0) as f32 + 0.3;

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
    mut char_q: Query<&mut Transform, With<CharacterMarker>>,
    height_noise: Res<HeightmapNoise>
){
    let mut char_pos = char_q.single_mut();
    for event in reader.read(){
        let x_pos = x_from_coord(event.to.0, event.to.1);
        let z_pos = z_from_coord(event.to.0, event.to.1);
        let y_pos =  height_noise.height_at_xz(x_pos, z_pos) + 0.3;
        char_pos.translation = vec3(x_pos, y_pos, z_pos);
    }

}