use std::{collections::HashSet, time::Instant};

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3A};
use winit::keyboard::KeyCode;

#[repr(C, align(16))]
#[derive(Clone, Copy, Pod, Zeroable, Default, Debug)]
pub struct Globals {
    pub mouse_pos: Vec2,
    pub resolution: [u32; 2],
    pub time_passed: f32,
    pub frame_time: f32,
    pub frame: u32,
    pub _pad: [f32; 1],
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Pod, Zeroable, Default, Debug)]
pub struct View {
    pub proj_view_rev_z: Mat4,
    pub inv_proj_view_rev_z: Mat4,
    pub proj_view: Mat4,
    pub inv_proj_view: Mat4,
    pub camera_position: Vec3A,
}

#[derive(Debug)]
pub struct Metadata {
    pub start_instant: Instant,
    pub prev_frame_start_insant: Instant,
    pub keyboard_state: HashSet<KeyCode>,
    pub delta_mouse: Vec2,
}

#[repr(C, align(64))]
#[derive(Clone, Copy, Pod, Zeroable, Default, Debug)]
pub struct VertexBuffer {
    pub position: Vec3A,
    pub normal: Vec3A,
    pub uv: Vec2,
    pub _pad: [f32; 6],
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            start_instant: Instant::now(),
            prev_frame_start_insant: Instant::now(),
            keyboard_state: HashSet::new(),
            delta_mouse: Vec2::ZERO,
        }
    }
}
