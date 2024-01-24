use std::f32::consts::PI;

use bevy::{math::{Vec3, Vec2, Vec3Swizzles, Vec2Swizzles}, transform::components::{GlobalTransform, Transform}, ecs::{system::{Query, Res, Commands}, query::With, component::Component, entity::Entity}, render::camera::Camera, input::{Input, keyboard::KeyCode}, pbr::{PointLightBundle, PointLight}, time::{Timer, Virtual, Time, TimerMode}, core_pipeline::core_3d::Camera3dBundle, app::{Plugin, Startup, Update}};
use bevy_mod_raycast::deferred::RaycastSource;

use crate::AddIfMissing;



pub struct CameraPlugin;
impl Plugin for CameraPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, camera_move)
        .add_systems(Update, rotate_camera);
    }
}



#[derive(Component)]
struct CameraMarker;

#[derive(Component)]
struct CameraFocus{
    pub location: Vec3
}

impl CameraFocus{
    fn new(location: Vec3) -> Self {
        Self {location}
    }

    /// TODO: !!!!!!!!!!!!!!!!! correct transform etc
    fn focus_camera_at(location: Vec3) -> (Camera3dBundle, CameraMarker, Self) {
        (
            Camera3dBundle {
                transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(location, Vec3::Y),
                ..Default::default()
            },
            CameraMarker,
            CameraFocus::new(location)
        )
    }

    ///this can probably be done better
    /// 
    /// Returns the delta that the camera should move to rotate around this point by the given angle
    fn rotate_around(&self, angle: f32, current_transform : Vec3) -> Vec3 {
        let relative = (current_transform-self.location).xz();
        let mut extended = (relative.rotate(Vec2::from_angle(angle)) - relative).xxy();
        extended.y = 0.0;
        extended
    }


}


#[derive(Component)]
struct CameraRotator{
    total_rotation: f32,
    current_rotation: f32,
    timer: Timer
}

impl CameraRotator{
    fn new(rotation: f32, secs: f32) -> Self{
        Self{total_rotation: rotation, current_rotation: 0.0, timer: Timer::from_seconds(secs, TimerMode::Once)}
    }


    fn current_delta(&self) -> f32 {
        self.total_rotation * self.timer.percent() - self.current_rotation
    }
}



fn rotate_camera(
    mut commands: Commands,
    mut camera_q: Query<(Entity, &GlobalTransform, &mut Transform, &CameraFocus, &mut CameraRotator), With<Camera>>,
    time: Res<Time<Virtual>>
) {
    for (ent, global, mut transform, focus, mut rotator) in camera_q.iter_mut() {
        //tick timer, then determine angle change and update it
        //then rotate by the angle change
        rotator.timer.tick(time.delta());
        let angle_change = rotator.current_delta();
        rotator.current_rotation += angle_change;
        let del = focus.rotate_around(angle_change, global.translation());
        transform.translation += del;
        transform.look_at(focus.location, Vec3::Y);
        if rotator.timer.just_finished(){
            commands.entity(ent).remove::<CameraRotator>();
        }
    }
}


fn spawn_camera(
    mut commands: Commands
){
    commands.spawn((
        CameraFocus::focus_camera_at(Vec3::new(0.0, 0.0, 0.0)),
        RaycastSource::<()>::new_cursor()
    ));

    //light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            range: 30.0,
            ..Default::default()
        },
        ..Default::default()
    });
}


fn camera_move(
    mut commands: Commands,
    mut camera_ent: Query<(Entity, &mut CameraFocus, &mut Transform), With<Camera>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time<Virtual>>
) {
    let (ent, mut focus, mut transform) = camera_ent.single_mut();

    let movement_dist = time.delta_seconds();
    let forward = transform.forward().reject_from_normalized(Vec3::Y);
    let left = transform.left().reject_from_normalized(Vec3::Y);

    if input.pressed(KeyCode::W){
        focus.location += movement_dist * forward;
        transform.translation += movement_dist * forward;
    }
    if input.pressed(KeyCode::S){
        focus.location += -movement_dist * forward;
        transform.translation += -movement_dist * forward;
    }
    if input.pressed(KeyCode::A){
        focus.location += movement_dist * left;
        transform.translation += movement_dist * left;
    }
    if input.pressed(KeyCode::D){
        focus.location += -movement_dist * left;
        transform.translation += -movement_dist * left;
    }

    if input.pressed(KeyCode::Q){
        commands.entity(ent).add(AddIfMissing{component: CameraRotator::new(0.25 * PI, 1.0)});
    }

    if input.pressed(KeyCode::E){
        commands.entity(ent).add(AddIfMissing{component: CameraRotator::new(-0.25 * PI, 1.0)});
    }
}