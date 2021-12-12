use bevy::{prelude::*, render::camera::Camera};

#[derive(Clone, Copy)]
pub struct CameraTarget {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    lerp_coeff: f32,
}

impl CameraTarget {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32, lerp_coeff: f32) -> Self {
        assert!(left <= right);
        assert!(top <= bottom);
        assert!(lerp_coeff > 0.0);
        Self {
            left,
            right,
            top,
            bottom,
            lerp_coeff,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(camera_follow_system.system());
    }
}

#[allow(clippy::type_complexity)]
fn camera_follow_system(
    mut query: QuerySet<(
        Query<&mut Transform, With<Camera>>,
        Query<(&Transform, &CameraTarget)>,
    )>,
) {
    let (&target_transform, &target_box) = match query.q1().iter().next() {
        Some(target) => target,
        _ => return,
    };

    for mut camera_transform in query.q0_mut().iter_mut() {
        let z = camera_transform.translation.z;
        let min = Vec3::new(
            target_transform.translation.x + target_box.left,
            target_transform.translation.y + target_box.top,
            z,
        );
        let max = Vec3::new(
            target_transform.translation.x + target_box.right,
            target_transform.translation.y + target_box.bottom,
            z,
        );

        camera_transform.translation = camera_transform.translation.lerp(
            camera_transform.translation.clamp(min, max),
            target_box.lerp_coeff,
        );
    }
}
