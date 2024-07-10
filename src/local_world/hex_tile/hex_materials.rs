use std::time::Duration;

use bevy::{
    app::{Plugin, Update}, asset::{Asset, Assets, Handle}, color::{Color, LinearRgba, Mix}, ecs::{component::Component, system::{Query, Res, ResMut}}, pbr::{
        Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin, StandardMaterial
    }, reflect::TypePath, render::{
        mesh::MeshVertexBufferLayoutRef, render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError
        }
    }, time::{self, Time, Timer, Virtual}
};


pub struct HexMaterialsPlugin;

impl Plugin for HexMaterialsPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_plugins(MaterialPlugin::<OutlineMaterial>::default())
        .add_systems(Update, update_colour_transitions);
    }
}


fn update_colour_transitions(
    mut handles: Query<(&Handle<StandardMaterial>, &mut ColourTransition)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time<Virtual>>
){
    let time_step = time.delta();

    for (handle, mut transition) in handles.iter_mut(){
        //update the colours
        match materials.get_mut(handle.id()){
            Some(material) => material.base_color = transition.get_interpolated_colour(),
            None => continue,
        }
        //tick the timers
        transition.tick(time_step)
    }
}




#[derive(Component)]
pub struct ColourTransition{
    start_colour: Color,
    end_colour: Color,
    timer: Timer
}

impl Default for ColourTransition{
    fn default() -> Self {
        Self { 
            start_colour: Color::linear_rgba(0.5, 0.5, 0.5, 1.0), //Grey
            end_colour: Color::linear_rgba(0.0, 1.0, 0.0, 1.0), //green
            timer: Timer::from_seconds(2.0, time::TimerMode::Once) 
        }
    }
}

impl ColourTransition{

    pub fn new(start_col: Color, end_col: Color, secs: f32) -> Self {
        Self {
            start_colour: start_col, 
            end_colour: end_col, 
            timer: Timer::from_seconds(secs, time::TimerMode::Once) 
        }
    }

    fn tick(&mut self, delta: Duration){
        self.timer.tick(delta);
    }

    pub fn get_interpolated_colour(&self) -> Color {
        self.start_colour.mix(&self.end_colour, self.timer.fraction())
    }
}



#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct OutlineMaterial{
    #[uniform(0)]
    pub outline_colour: LinearRgba,
}

impl Material for OutlineMaterial{
    fn fragment_shader() -> ShaderRef{
        "shaders/outline_material.wgsl".into()
    }
    
    fn depth_bias(&self) -> f32 {
        1.0
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

