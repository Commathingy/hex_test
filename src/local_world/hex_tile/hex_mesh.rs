use bevy::{app::{Plugin, PreStartup}, asset::{Assets, Handle}, color::LinearRgba, ecs::system::{Commands, ResMut, Resource}, render::{mesh::{Indices, Mesh}, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology}};

use super::hex_materials::OutlineMaterial;


pub struct HexMeshPlugin;
impl Plugin for HexMeshPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, create_handles);
    }
}



fn create_handles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut outline_materials: ResMut<Assets<OutlineMaterial>>,
) {
    commands.insert_resource(HexagonMeshHandles{
        hex_mesh: meshes.add(create_hex_mesh()),
        outline_mesh: meshes.add(create_outline_mesh()),
        outline_material: outline_materials.add(OutlineMaterial{outline_colour: LinearRgba::new(1.0, 1.0, 1.0, 1.0)})
    });
}

#[derive(Resource)]
pub struct HexagonMeshHandles{
    pub hex_mesh: Handle<Mesh>,
    pub outline_mesh: Handle<Mesh>,
    pub outline_material: Handle<OutlineMaterial>
}


fn create_hex_mesh() -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all())
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        Vec::from(VERTEX_POSITIONS)
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        Vec::from(VERTEX_UVS)
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        Vec::from(VERTEX_NORMALS)
        )
    .with_inserted_indices(Indices::U32(Vec::from(VERTEX_INDICES)))
}

fn create_outline_mesh() -> Mesh {
    Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION, 
        Vec::from(OUTLINE_POSITIONS)
    )
    .with_inserted_indices(Indices::U32(Vec::from(OUTLINE_INDICES)))
}


pub const FRAC_1_SQRT_3: f32 = 0.577350269189625764509148780501957456_f32;
const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;

const MINOR_RADIUS : f32 = 0.5;
//const MAJOR_RADIUS : f32 = FRAC_1_SQRT_3; // = 2/sqrt(3) * minor_radius

const HEIGHT : f32 = 2.0;

const X : f32 = MINOR_RADIUS * FRAC_1_SQRT_3;
const Z : f32 = MINOR_RADIUS;


pub const VERTEX_POSITIONS : [[f32; 3]; 36] = [
            [2.0*X, 0.0, 0.0],      //v0    0
            [X, 0.0, Z],            //v1    1
            [-X, 0.0, Z],           //v2    2
            [-2.0*X, 0.0, 0.0],     //v3    3
            [-X, 0.0, -Z],          //v4    4
            [X, 0.0, -Z],           //v5    5

            //Bottom Face Vertices
            [2.0*X, -HEIGHT, 0.0],  //v6    6
            [X, -HEIGHT, Z],        //v7    7
            [-X, -HEIGHT, Z],       //v8    8
            [-2.0*X, -HEIGHT, 0.0], //v9    9
            [-X, -HEIGHT, -Z],      //v10   10
            [X, -HEIGHT, -Z],       //v11   11

            //Side faces vertices
            //1
            [2.0*X, 0.0, 0.0],      //v0    12
            [X, 0.0, Z],            //v1    13
            [2.0*X, -HEIGHT, 0.0],  //v6    14
            [X, -HEIGHT, Z],        //v7    15
            //2
            [X, 0.0, Z],            //v1    16
            [-X, 0.0, Z],           //v2    17
            [X, -HEIGHT, Z],        //v7    18
            [-X, -HEIGHT, Z],       //v8    19
            //3
            [-X, 0.0, Z],           //v2    20
            [-2.0*X, 0.0, 0.0],     //v3    21
            [-X, -HEIGHT, Z],       //v8    22
            [-2.0*X, -HEIGHT, 0.0], //v9    23
            //4
            [-2.0*X, 0.0, 0.0],     //v3    24
            [-X, 0.0, -Z],          //v4    25
            [-2.0*X, -HEIGHT, 0.0], //v9    26
            [-X, -HEIGHT, -Z],      //v10   27
            //5
            [-X, 0.0, -Z],          //v4    28
            [X, 0.0, -Z],           //v5    29
            [-X, -HEIGHT, -Z],      //v10   30
            [X, -HEIGHT, -Z],       //v11   31
            //6
            [X, 0.0, -Z],           //v5    32
            [2.0*X, 0.0, 0.0],      //v0    33
            [X, -HEIGHT, -Z],       //v11   34
            [2.0*X, -HEIGHT, 0.0]   //v6    35
];

pub const VERTEX_UVS : [[f32; 2]; 36] = [
    [0.0, 0.5],                     //0
    [0.25*FRAC_1_SQRT_3, 0.75],     //1
    [0.75*FRAC_1_SQRT_3, 0.75],     //2
    [FRAC_1_SQRT_3, 0.5],           //3
    [0.75*FRAC_1_SQRT_3, 0.25],     //4
    [0.25*FRAC_1_SQRT_3, 0.25],     //5

    //bottom face    todo->flip order?
    [0.0, 0.5],                     //6
    [0.25*FRAC_1_SQRT_3, 0.75],     //7
    [0.75*FRAC_1_SQRT_3, 0.75],     //8
    [FRAC_1_SQRT_3, 0.5],           //9
    [0.75*FRAC_1_SQRT_3, 0.25],     //10
    [0.25*FRAC_1_SQRT_3, 0.25],     //11

    //sides           todo->flip order?
    //1
    [1.0-0.5*FRAC_1_SQRT_3, 0.0],   //12
    [1.0, 0.0],                     //13
    [1.0-0.5*FRAC_1_SQRT_3, 1.0],   //14
    [1.0, 1.0],                     //15
    //2
    [1.0-0.5*FRAC_1_SQRT_3, 0.0],   //16
    [1.0, 0.0],                     //17
    [1.0-0.5*FRAC_1_SQRT_3, 1.0],   //18
    [1.0, 1.0],                     //19
    //3
    [1.0-0.5*FRAC_1_SQRT_3, 0.0],   //20
    [1.0, 0.0],                     //21
    [1.0-0.5*FRAC_1_SQRT_3, 1.0],   //22
    [1.0, 1.0],                     //23
    //4
    [1.0-0.5*FRAC_1_SQRT_3, 0.0],   //24
    [1.0, 0.0],                     //25
    [1.0-0.5*FRAC_1_SQRT_3, 1.0],   //26
    [1.0, 1.0],                     //27
    //5
    [1.0-0.5*FRAC_1_SQRT_3, 0.0],   //28
    [1.0, 0.0],                     //29
    [1.0-0.5*FRAC_1_SQRT_3, 1.0],   //30
    [1.0, 1.0],                     //31
    //6
    [1.0-0.5*FRAC_1_SQRT_3, 0.0],   //32
    [1.0, 0.0],                     //33
    [1.0-0.5*FRAC_1_SQRT_3, 1.0],   //34
    [1.0, 1.0],                     //35
];

pub const VERTEX_NORMALS : [[f32; 3]; 36] = [
    [0.0, 1.0, 0.0],        //v0    0
    [0.0, 1.0, 0.0],        //v1    1
    [0.0, 1.0, 0.0],        //v2    2
    [0.0, 1.0, 0.0],        //v3    3
    [0.0, 1.0, 0.0],        //v4    4
    [0.0, 1.0, 0.0],        //v5    5

    //Bottom Face Vertices
    [0.0, -1.0, 0.0],       //v6    6
    [0.0, -1.0, 0.0],       //v7    7
    [0.0, -1.0, 0.0],       //v8    8
    [0.0, -1.0, 0.0],       //v9    9
    [0.0, -1.0, 0.0],       //v10   10
    [0.0, -1.0, 0.0],       //v11   11

    //Side faces vertices
    //1
    [0.5 * SQRT_3, 0.0, 0.5],   //v0    12
    [0.5 * SQRT_3, 0.0, 0.5],   //v1    13
    [0.5 * SQRT_3, 0.0, 0.5],   //v6    14
    [0.5 * SQRT_3, 0.0, 0.5],   //v7    15
    //2
    [0.0, 0.0, 1.0],            //v1    16
    [0.0, 0.0, 1.0],            //v2    17
    [0.0, 0.0, 1.0],            //v7    18
    [0.0, 0.0, 1.0],            //v8    19
    //3
    [-0.5 * SQRT_3, 0.0, 0.5],  //v2    20
    [-0.5 * SQRT_3, 0.0, 0.5],  //v3    21
    [-0.5 * SQRT_3, 0.0, 0.5],  //v8    22
    [-0.5 * SQRT_3, 0.0, 0.5],  //v9    23
    //4
    [-0.5 * SQRT_3, 0.0, -0.5], //v3    24
    [-0.5 * SQRT_3, 0.0, -0.5], //v4    25
    [-0.5 * SQRT_3, 0.0, -0.5], //v9    26
    [-0.5 * SQRT_3, 0.0, -0.5], //v10   27
    //5
    [0.0, 0.0, -1.0],           //v4    28
    [0.0, 0.0, -1.0],           //v5    29
    [0.0, 0.0, -1.0],           //v10   30
    [0.0, 0.0, -1.0],           //v11   31
    //6
    [0.5 * SQRT_3, 0.0, -0.5],  //v5    32
    [0.5 * SQRT_3, 0.0, -0.5],  //v0    33
    [0.5 * SQRT_3, 0.0, -0.5],  //v11   34
    [0.5 * SQRT_3, 0.0, -0.5]   //v6    35
];

pub const VERTEX_INDICES : [u32; 60]= [
        0, 2, 1,
        0, 3, 2,
        0, 5, 3,
        3, 5, 4,

        // Bottom Face
        8, 6, 7,
        9, 6, 8,
        11, 6, 9,
        11, 9, 10,

        // Side Faces
        //1
        12, 13, 14,
        13, 15, 14,

        //2
        16, 17, 18,
        17, 19, 18,

        //3
        20, 21, 22,
        21, 23, 22,

        //4
        24, 25, 26,
        25, 27, 26,

        //5
        28, 29, 30,
        29, 31, 30,

        //6
        32, 33, 34,
        33, 35, 34,
];

pub const OUTLINE_POSITIONS : [[f32; 3]; 12] = [
    [2.0*X, 0.0, 0.0],      //v0
    [X, 0.0, Z],            //v1
    [-X, 0.0, Z],           //v2
    [-2.0*X, 0.0, 0.0],     //v3
    [-X, 0.0, -Z],          //v4
    [X, 0.0, -Z],           //v5

    //Bottom Face Vertice
    [2.0*X, -HEIGHT, 0.0],  //v6
    [X, -HEIGHT, Z],        //v7
    [-X, -HEIGHT, Z],       //v8
    [-2.0*X, -HEIGHT, 0.0], //v9
    [-X, -HEIGHT, -Z],      //v10
    [X, -HEIGHT, -Z],       //v11
];

pub const OUTLINE_INDICES : [u32; 36] = [
    //Top face lines
    0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 0,
    //Bottom face lines
    6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 6,
    //Side face lines
    0, 6, 1, 7, 2, 8, 3, 9, 4, 10, 5, 11
];

