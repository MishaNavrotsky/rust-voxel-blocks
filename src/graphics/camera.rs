use glam::{Mat4, Quat, Vec2, Vec3};
use winit::event::KeyEvent;

use crate::graphics::structures::View;

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,
    pub fov: f32,
    pub aspect_ratio: f32,

    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        return Camera {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            fov: 90.0_f32.to_radians(),
            aspect_ratio,
            yaw: 0.0,
            pitch: 0.0,
            speed: 5.0,
            sensitivity: 0.002,
        };
    }
    pub fn get_view(&self) -> View {
        let proj_view_rev_z =
            Mat4::perspective_infinite_reverse_rh(self.fov, self.aspect_ratio, 0.001)
                * Mat4::from_rotation_translation(self.rotation, self.position).inverse();

        let proj_view = Mat4::perspective_infinite_rh(self.fov, self.aspect_ratio, 0.001)
            * Mat4::from_rotation_translation(self.rotation, self.position).inverse();
        View {
            proj_view_rev_z,
            inv_proj_view_rev_z: proj_view_rev_z.inverse(),
            proj_view,
            inv_proj_view: proj_view.inverse(),
            camera_position: self.position.into(),
        }
    }

    pub fn update_rotation(&mut self, delta_rotation: Vec2) {
        self.yaw -= delta_rotation.x * self.sensitivity;
        self.pitch -= delta_rotation.y * self.sensitivity;
        self.pitch = self.pitch.clamp(-1.57, 1.57);

        self.rotation =
            Quat::from_axis_angle(Vec3::Y, self.yaw) * Quat::from_axis_angle(Vec3::X, self.pitch);
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::new(0.0, 0.0, -1.0)
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::new(1.0, 0.0, 0.0)
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }
}
