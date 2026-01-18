use wgpu::*;

use crate::graphics::{bind_group_layouts, buffers};

pub struct BindGroups {
    pub globals: BindGroup,
    pub bview: BindGroup,
    pub vertices: BindGroup,
}

impl BindGroups {
    pub fn new(
        device: &Device,
        bind_group_layouts: &bind_group_layouts::BindGroupLayouts,
        buffers: &buffers::Buffers,
    ) -> Self {
        Self {
            globals: device.create_bind_group(&BindGroupDescriptor {
                label: Some("Bind Group Globals"),
                layout: &bind_group_layouts.globals,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &buffers.globals,
                        offset: 0,
                        size: None,
                    }),
                }],
            }),
            bview: device.create_bind_group(&BindGroupDescriptor {
                label: Some("Bind Group View"),
                layout: &bind_group_layouts.view,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &buffers.view,
                        offset: 0,
                        size: None,
                    }),
                }],
            }),
            vertices: device.create_bind_group(&BindGroupDescriptor {
                label: Some("Bind Group Vertices"),
                layout: &bind_group_layouts.vertices,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &buffers.vertices,
                        offset: 0,
                        size: None,
                    }),
                }],
            }),
        }
    }

    pub fn as_slice(&self) -> [&BindGroup; 3] {
        [&self.globals, &self.bview, &self.vertices]
    }
}
