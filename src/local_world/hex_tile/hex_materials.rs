use std::time::Duration;

use bevy::{
    render::{
        render_resource::{
            ShaderRef, 
            AsBindGroup, 
            RenderPipelineDescriptor, 
            SpecializedMeshPipelineError, 
            PolygonMode
        }, 
        color::Color, 
        mesh::MeshVertexBufferLayout
    }, 
    pbr::{
        Material, 
        MaterialPipeline, 
        MaterialPipelineKey, 
        StandardMaterial, 
        MaterialPlugin
    }, 
    asset::{Asset, Handle, Assets}, 
    reflect::TypePath, app::{Plugin, Update}, ecs::{system::{ResMut, Query, Res}, component::Component}, time::{self, Timer, Time, Virtual}
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
        match materials.get_mut(handle.clone()){
            Some(material) => material.base_color = transition.get_interpolated_colour(),
            None => continue,
        }
        //tick the timers
        transition.tick(time_step)
    }
}


pub enum ColourSpace{
    RGB,
    RGBLinear,
    HSL,
    LCH
}


#[derive(Component)]
pub struct ColourTransition{
    interpolation_type: ColourSpace,
    start_colour: Color,
    end_colour: Color,
    timer: Timer
}

impl Default for ColourTransition{
    fn default() -> Self {
        Self {
            interpolation_type: ColourSpace::RGB, 
            start_colour: Color::GRAY, 
            end_colour: Color::GREEN, 
            timer: Timer::from_seconds(2.0, time::TimerMode::Once) 
        }
    }
}

impl ColourTransition{

    pub fn new(start_col: Color, end_col: Color, secs: f32) -> Self {
        Self {
            interpolation_type: ColourSpace::RGB, 
            start_colour: start_col, 
            end_colour: end_col, 
            timer: Timer::from_seconds(secs, time::TimerMode::Once) 
        }
    }

    fn tick(&mut self, delta: Duration){
        self.timer.tick(delta);
    }

    fn get_interpolated_colour(&self) -> Color {
        let (start, end) = match self.interpolation_type{
            ColourSpace::RGB => (self.start_colour.as_rgba_f32(), self.end_colour.as_rgba_f32()),
            ColourSpace::RGBLinear => (self.start_colour.as_linear_rgba_f32(), self.end_colour.as_linear_rgba_f32()),
            ColourSpace::HSL => (self.start_colour.as_hsla_f32(), self.end_colour.as_hsla_f32()),
            ColourSpace::LCH => (self.start_colour.as_lcha_f32(), self.end_colour.as_lcha_f32()),
        };
        let ratio = self.timer.percent();
        let mut fv: [f32; 4] = [0.0; 4];
        for i in 0..4 {
            fv[i] = start[i] + (end[i]-start[i]) * ratio;
        }
        match self.interpolation_type{
            ColourSpace::RGB => Color::rgba(fv[0], fv[1], fv[2], fv[3]),
            ColourSpace::RGBLinear => Color::rgba_linear(fv[0], fv[1], fv[2], fv[3]),
            ColourSpace::HSL => Color::hsla(fv[0], fv[1], fv[2], fv[3]),
            ColourSpace::LCH => Color::lcha(fv[0], fv[1], fv[2], fv[3]),
        }
    }
}



#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct OutlineMaterial{
    #[uniform(0)]
    pub outline_colour: Color,
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
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

