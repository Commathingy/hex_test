use std::f32::consts::PI;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::math::vec3;
use bevy::render::camera::{OrthographicProjection, ScalingMode};
use bevy::{math::Vec3, transform::components::Transform, ecs::{system::{Query, Res, Commands}, query::With, component::Component}, render::camera::Camera, input::{ButtonInput, keyboard::KeyCode}, pbr::{PointLightBundle, PointLight}, time::{Virtual, Time}, core_pipeline::core_3d::Camera3dBundle, app::{Plugin, Startup, Update}};
use bevy_mod_raycast::deferred::RaycastSource;

pub struct LocalCameraPlugin;
impl Plugin for LocalCameraPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, camera_move);
    }
}



#[derive(Component)]
struct CameraMarker;

#[derive(Component)]
struct CameraFocus{
    pub location: Vec3,
    distance: f32,
    angle: Angle,
    inclement: Angle,
    rotation_speed: f32
}

impl CameraFocus{
    fn new(location: Vec3) -> Self {
        Self {location, distance: 20.0, angle: Angle(0.0), inclement: Angle(PI / 8.0), rotation_speed: PI}
    }

    fn focus_camera_at(location: Vec3) -> (Camera3dBundle, CameraMarker, Self) {
        (
            Camera3dBundle {
                projection: OrthographicProjection {
                    // 6 world units per window height.
                    scaling_mode: ScalingMode::WindowSize(64.0),
                    ..Default::default()
                }.into(),
                ..Default::default()
            },
            CameraMarker,
            CameraFocus::new(location)
        )
    }

    ///compute the new position of the camera around the focus from its current angle
    fn determine_position(&self) -> Transform {
        let (new_y, xz_scale) = self.inclement.0.sin_cos();
        let (new_z_unscale, new_x_unscale) = self.angle.0.sin_cos();
        let new_pos = self.location + self.distance * vec3(new_x_unscale * xz_scale, new_y, new_z_unscale * xz_scale);

        let mut transform = Transform::from_translation(new_pos);
        transform.look_at(self.location, Vec3::Y);
        transform
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
    mut camera_ent: Query<(&mut CameraFocus, &mut Transform), With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Virtual>>
) {
    let (mut focus, mut transform) = camera_ent.single_mut();

    let time_pass = time.delta_seconds();
    let forward = transform.forward().reject_from_normalized(Vec3::Y);
    let left = transform.left().reject_from_normalized(Vec3::Y);
    let rot_speed = focus.rotation_speed;

    if input.pressed(KeyCode::KeyW){
        focus.location += 4.0 * time_pass * forward;
    }
    if input.pressed(KeyCode::KeyS){
        focus.location += -4.0 * time_pass * forward;
    }
    if input.pressed(KeyCode::KeyA){
        focus.location += 4.0 * time_pass * left;
    }
    if input.pressed(KeyCode::KeyD){
        focus.location += -4.0 * time_pass * left;
    }
    if input.pressed(KeyCode::KeyQ){
        focus.angle += Angle(time_pass * rot_speed);
    }
    if input.pressed(KeyCode::KeyE){
        focus.angle -= Angle(time_pass * rot_speed);
    }
    if input.pressed(KeyCode::KeyR){
        focus.inclement += Angle(time_pass * rot_speed);
    }
    if input.pressed(KeyCode::KeyF){
        focus.inclement -= Angle(time_pass * rot_speed);
    }
    if input.pressed(KeyCode::Space){
        println!("ang: {}, inc: {}", focus.angle.0, focus.inclement.0);
    }

    //actually effect the changes
    *transform = focus.determine_position();
}

#[derive(PartialEq)]
struct Angle(pub f32);

impl Add<&Angle> for &Angle{
    type Output = Angle;

    fn add(self, rhs: &Angle) -> Self::Output {
        Angle((self.0 + rhs.0) % (2.0 * PI))
    }
}

impl AddAssign<Angle> for Angle{
    fn add_assign(&mut self, rhs: Angle) {
        self.0 = (self.0 + rhs.0) % (2.0 * PI);
    }
}

impl Sub<&Angle> for &Angle{
    type Output = Angle;

    fn sub(self, rhs: &Angle) -> Self::Output {
        Angle((self.0 - rhs.0 + 2.0 * PI) % (2.0 * PI))
    }
}

impl SubAssign<Angle> for Angle{
    fn sub_assign(&mut self, rhs: Angle) {
        self.0 = (self.0 - rhs.0 + 2.0 * PI) % (2.0 * PI);
    }
}