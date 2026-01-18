use wgpu::*;

use crate::graphics::structures::VertexBuffer;

pub struct RenderPass {
    pipeline: RenderPipeline,
}

impl RenderPass {
    pub fn new(device: &Device, bind_group_layouts: &[&BindGroupLayout]) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pass Pipeline Layout"),
            bind_group_layouts: bind_group_layouts,
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pass Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &device.create_shader_module(ShaderModuleDescriptor {
                    label: Some("Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_str!("../assets/shaders/render_pass.wgsl").into(),
                    ),
                }),
                entry_point: Some("main_vertex"),
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<VertexBuffer>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            format: VertexFormat::Float32x4, //position
                            offset: 0,
                            shader_location: 0,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x4, //normal
                            offset: std::mem::offset_of!(VertexBuffer, normal) as u64,
                            shader_location: 1,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x2, //uv
                            offset: std::mem::offset_of!(VertexBuffer, uv) as u64,
                            shader_location: 2,
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Fragment Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_str!("../assets/shaders/render_pass.wgsl").into(),
                    ),
                }),
                entry_point: Some("main_fragment"),
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                cull_mode: Some(Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::GreaterEqual,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState::default(),
            multiview: None,
            cache: Default::default(),
        });

        Self { pipeline }
    }
    pub fn encode(
        &self,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        depth_view: &TextureView,
        bind_groups: &[&BindGroup],
        vertex_buffer: &Buffer,
    ) {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass Descriptor"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(0.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        for (i, bind_group) in bind_groups.iter().enumerate() {
            render_pass.set_bind_group(i as u32, *bind_group, &[]);
        }
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(
            0..(vertex_buffer.size() / std::mem::size_of::<VertexBuffer>() as u64) as u32,
            0..1,
        );
    }
}
