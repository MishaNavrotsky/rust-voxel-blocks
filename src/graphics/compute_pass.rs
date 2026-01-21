use wgpu::*;

pub struct ComputePass {
    pipeline: wgpu::ComputePipeline,
}

impl ComputePass {
    pub fn new() -> Self {
        todo!()
    }

    pub fn encode(&self, encoder: &mut CommandEncoder) {
        todo!()
    }
}
