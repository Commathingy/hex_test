mod hex_mesh;
mod hex_materials;
mod local_terrain;

use bevy::{
    app::
        Plugin, 
        asset::Assets, 
        color::Color, 
        ecs::{component::Component, entity::Entity, event::{Event, EventReader, EventWriter}, 
        schedule::{apply_deferred, IntoSystemConfigs}, 
        system::{Commands, Query, Res, ResMut, Resource}}, 
        hierarchy::BuildChildren, math::Vec3, 
        pbr::{MaterialMeshBundle, StandardMaterial}, 
        prelude::{SpatialBundle, SystemSet}, 
        state::state::OnEnter, 
        transform::components::Transform, 
        utils::hashbrown::HashMap
};

use bevy_mod_raycast::deferred::RaycastMesh;


use hex_materials::HexMaterialsPlugin;
pub use hex_materials::ColourTransition;
use local_terrain::TerrainPlugin;
use crate::{graph_functions::GraphVertex, GameState};
use self::hex_mesh::{HexMeshPlugin, HexagonMeshHandles};
pub use self::hex_mesh::{x_from_coord, z_from_coord};


pub struct HexPlugin;
impl Plugin for HexPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_plugins(HexMaterialsPlugin)
        .add_plugins(HexMeshPlugin)
        .add_plugins(TerrainPlugin)
        .init_resource::<HexagonsToUpdate>()
        .init_resource::<HexPositionMap>()
        .add_event::<SpawnHexEvent>()
        .add_systems(OnEnter(GameState::LocalWorld), 
    (
                setup_hexes, 
                apply_deferred, 
                spawn_hexes, 
                apply_deferred, 
                apply_neighbour_changes
            ).chain().in_set(HexSpawnSet)
        );
    }
}



//====================
// Systems
//====================

fn apply_neighbour_changes(
    mut to_update: ResMut<HexagonsToUpdate>,
    mut hexes: Query<&mut HexTile>
){
    for (ent, mut neighbours) in to_update.0.drain(){
        match hexes.get_mut(ent){
            Ok(mut tile) => tile.neighbours.append(&mut neighbours),
            Err(_) => continue,
        }
    }
}

pub fn hex_neighbours(pos: &(i32, i32)) -> Vec<(i32,i32)> {
    // 0 1
    //2 . 3
    // 4 5
    const POSSIBLE_NEIGHBOURS : [(i32, i32); 6] = [
        (1, 0), (1, -1), 
        (0, 1), (0, -1), 
        (-1, 0), (-1, -1)
    ];
    let mult = if pos.0 % 2 == 0 {1} else {-1};
    POSSIBLE_NEIGHBOURS.into_iter().map(|(x, y)| (pos.0 + mult * x, pos.1 + mult * y)).collect()
}

fn setup_hexes(
    mut writer: EventWriter<SpawnHexEvent>
){

    //For a hexagonal grid with radius of hex_size
    let hex_size: i32 = 25;
    for i in -hex_size..=hex_size {
        let min_j = (i.abs() / 2) - hex_size;
        let max_j = hex_size - ((i.abs() + 1) / 2);
        for j in min_j..=max_j{
            writer.send(SpawnHexEvent{
                position: (i ,j)
            });
        }
    }


    /* //For a simple rectangular grid
    for i in -10..11{
        for j in -10..11{
            writer.send(SpawnHexEvent{
                position: (i ,j)
            });
        }
    }
    */
}

fn spawn_hexes(
    mut commands: Commands,
    mut reader: EventReader<SpawnHexEvent>,
    mut tile_map: ResMut<HexPositionMap>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    handles: Res<HexagonMeshHandles>,
    mut hex_to_update: ResMut<HexagonsToUpdate>
) {
    //stores the neighbours that will be added to the entity in a later system
    //added to when we add a vertex that neighbours this one, since we cant add it then
    let mut to_update: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for event in reader.read(){
        let mut neighbours : Vec<Entity> = Vec::new();
        //iterate over the possible neighbours
        for other_coord in hex_neighbours(&event.position).iter(){
            //check if they exist already
            if let Some(&ent) = tile_map.map.get(other_coord){
                //if it does already exist, add it as a neighbour
                neighbours.push(ent);
            }
        }

        let x_pos = x_from_coord(event.position.0, event.position.1);
        let z_pos = z_from_coord(event.position.0, event.position.1);




        let new_id = commands.spawn((
            HexTile{
                position: event.position,
                neighbours: neighbours.clone(),
                explored_state: TileExploredState::Hidden
            },
            SpatialBundle::from(Transform::from_translation(Vec3::new(x_pos, 0.0, z_pos)))

        )).with_children(|par| {
            par.spawn((
                MaterialMeshBundle{
                    mesh: handles.hex_mesh.clone(),
                    material: materials.add(StandardMaterial::from(Color::linear_rgba(0.0, 0.0, 0.0, 1.0))),
                    ..Default::default()
                },
                RaycastMesh::<()>::default()
            ));
            par.spawn(
                MaterialMeshBundle{
                    mesh: handles.outline_mesh.clone(),
                    material: handles.outline_material.clone(),
                    ..Default::default()
                }
            );
        }).id();


        //update the to_update list of each of those neighbours
        for ent in neighbours.into_iter(){
            to_update.entry(ent).and_modify(|vec| vec.push(new_id)).or_insert(vec![new_id]);
        }

        tile_map.map.insert(event.position, new_id);
    }
    *hex_to_update = HexagonsToUpdate(to_update);
}

// ========================
// Types
// ========================


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexSpawnSet;



#[derive(Resource, Default)]
pub struct HexPositionMap{
    pub map: HashMap<(i32, i32), Entity>
}


#[derive(Event)]
struct SpawnHexEvent{
    pub position: (i32, i32)
}

#[derive(Resource, Default)]
struct HexagonsToUpdate(pub HashMap<Entity, Vec<Entity>>);


#[derive(Component)]
pub struct HexTile{
    pub position: (i32, i32),
    pub neighbours: Vec<Entity>,
    pub explored_state: TileExploredState
}

impl GraphVertex for HexTile{
    fn get_neighbours(&self) -> Vec<Entity> {
        self.neighbours.clone()
    }
}

pub enum TileExploredState{
    Hidden,
    Explored,
    Visible
}



