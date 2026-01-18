use wgpu::*;

use crate::graphics::structures;

pub struct Buffers {
    pub globals: Buffer,
    pub view: Buffer,
    pub vertices: Buffer,
}

impl Buffers {
    pub fn new(device: &wgpu::Device, vertex_count: u64) -> Self {
        Self {
            globals: device.create_buffer(&BufferDescriptor {
                label: Some("Globals Buffer"),
                size: std::mem::size_of::<structures::Globals>() as u64,
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            view: device.create_buffer(&BufferDescriptor {
                label: Some("View Buffer"),
                size: std::mem::size_of::<structures::View>() as u64,
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            vertices: device.create_buffer(&BufferDescriptor {
                label: Some("Vertices Buffer"),
                size: std::mem::size_of::<structures::VertexBuffer>() as u64 * vertex_count,
                usage: BufferUsages::VERTEX | BufferUsages::STORAGE | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
        }
    }
}
