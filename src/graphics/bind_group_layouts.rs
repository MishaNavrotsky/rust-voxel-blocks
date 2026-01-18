use wgpu::*;

pub enum BindGroupUsage {
    Compute,
    Render,
}
pub struct BindGroupLayouts {
    pub globals: BindGroupLayout,
    pub view: BindGroupLayout,
    pub vertices: BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &Device, usage: BindGroupUsage) -> Self {
        let (visibility, storage_ro) = match usage {
            BindGroupUsage::Compute => (
                ShaderStages::COMPUTE,
                false, // RW
            ),
            BindGroupUsage::Render => (
                ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                true, // RO
            ),
        };

        Self {
            globals: device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout Globals"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            }),
            view: device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout View"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            }),
            vertices: device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout Vertices"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage {
                            read_only: storage_ro,
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            }),
        }
    }

    pub fn as_slice(&self) -> [&BindGroupLayout; 3] {
        [&self.globals, &self.view, &self.vertices]
    }
}
