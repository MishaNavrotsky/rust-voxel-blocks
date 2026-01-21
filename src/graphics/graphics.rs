use std::{sync::Arc, time::Instant};

use glam::Vec2;
use wgpu::*;
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy, keyboard::PhysicalKey, window::Window};

use crate::graphics::{
    bind_group_layouts, bind_groups, buffers,
    camera::Camera,
    render_pass,
    structures::{Globals, Metadata, View},
};

pub async fn create_graphics(window: Arc<Window>, proxy: EventLoopProxy<Graphics>) {
    let instance = Instance::default();
    let surface = instance.create_surface(Arc::clone(&window)).unwrap();

    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Could not get an adapter (GPU).");

    let (device, queue) = adapter
        .request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: adapter.limits(),
            memory_hints: MemoryHints::Performance,
            trace: Default::default(),
            experimental_features: ExperimentalFeatures::default(),
        })
        .await
        .expect("Failed to get device");

    let size = window.inner_size();
    let width = size.width.max(1);
    let height = size.height.max(1);
    let surface_config = surface.get_default_config(&adapter, width, height).unwrap();

    surface.configure(&device, &surface_config);

    let (depth_texture, depth_texture_view) = create_depth_texture(&device, &surface_config);

    let bind_group_layouts_compute = bind_group_layouts::BindGroupLayouts::new(
        &device,
        bind_group_layouts::BindGroupUsage::Compute,
    );
    let bind_group_layouts_render = bind_group_layouts::BindGroupLayouts::new(
        &device,
        bind_group_layouts::BindGroupUsage::Render,
    );
    let render_pass = render_pass::RenderPass::new(&device, &bind_group_layouts_render.as_slice());
    let buffers = buffers::Buffers::new(&device, 3);
    let bind_groups_compute =
        bind_groups::BindGroups::new(&device, &bind_group_layouts_compute, &buffers);
    let bind_groups_render =
        bind_groups::BindGroups::new(&device, &bind_group_layouts_render, &buffers);

    let gfx = Graphics {
        window: window.clone(),
        instance,
        surface,
        surface_config,
        adapter,
        device,
        queue,

        depth_texture,
        depth_texture_view,

        camera: Camera::new(width as f32 / height as f32),
        metadata: Metadata::new(),
        globals: Globals {
            resolution: [width, height],
            ..Default::default()
        },
        view: Default::default(),

        render_pass,
        buffers,
        bind_group_layouts_compute,
        bind_group_layouts_render,
        bind_groups_compute,
        bind_groups_render,
    };

    let _ = proxy.send_event(gfx);
}

fn create_depth_texture(
    device: &Device,
    surface_config: &wgt::SurfaceConfiguration<Vec<TextureFormat>>,
) -> (Texture, TextureView) {
    let depth_texture = device.create_texture(&TextureDescriptor {
        label: Some("Depth Texture"),
        size: Extent3d {
            width: surface_config.width,
            height: surface_config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Depth32Float,
        usage: TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let depth_texture_view = depth_texture.create_view(&TextureViewDescriptor::default());
    (depth_texture, depth_texture_view)
}

pub struct Graphics {
    pub window: Arc<Window>,
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,

    pub depth_texture: Texture,
    pub depth_texture_view: TextureView,

    pub camera: Camera,
    pub metadata: Metadata,
    pub globals: Globals,
    pub view: View,

    pub render_pass: render_pass::RenderPass,
    pub buffers: buffers::Buffers,
    pub bind_group_layouts_compute: bind_group_layouts::BindGroupLayouts,
    pub bind_group_layouts_render: bind_group_layouts::BindGroupLayouts,
    pub bind_groups_compute: bind_groups::BindGroups,
    pub bind_groups_render: bind_groups::BindGroups,
}

impl Graphics {
    pub fn set_mouse_pos(&mut self, mouse_pos: Vec2) {
        self.globals.mouse_pos = mouse_pos.into();
    }

    pub fn handle_mouse_motion(&mut self, delta_x: f32, delta_y: f32) {
        self.metadata.delta_mouse += Vec2::new(delta_x, delta_y);
    }

    pub fn handle_keyboard_input(&mut self, event: &winit::event::KeyEvent) {
        let PhysicalKey::Code(key_code) = event.physical_key else {
            return;
        };

        if event.state == winit::event::ElementState::Pressed {
            self.metadata.keyboard_state.insert(key_code);
        } else {
            self.metadata.keyboard_state.remove(&key_code);
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);

        self.camera = Camera {
            aspect_ratio: self.surface_config.width as f32 / self.surface_config.height as f32,
            ..self.camera
        };

        self.globals.resolution = [self.surface_config.width, self.surface_config.height];

        let (depth_texture, depth_texture_view) =
            create_depth_texture(&self.device, &self.surface_config);
        self.depth_texture = depth_texture;
        self.depth_texture_view = depth_texture_view;
    }

    pub fn run_cs(&self, command_encoder: &mut CommandEncoder) {}

    pub fn run_rs(&self, command_encoder: &mut CommandEncoder, frame: &mut SurfaceTexture) {
        self.render_pass.encode(
            command_encoder,
            &frame.texture.create_view(&TextureViewDescriptor::default()),
            &self.depth_texture_view,
            &self.bind_groups_render.as_slice(),
            &self.buffers.vertices,
        );
    }

    pub fn update_uniforms(&self) {
        self.queue
            .write_buffer(&self.buffers.globals, 0, bytemuck::bytes_of(&self.globals));
        self.queue
            .write_buffer(&self.buffers.view, 0, bytemuck::bytes_of(&self.view));
    }

    pub fn update(&mut self) {
        let delta_time = (Instant::now() - self.metadata.prev_frame_start_insant).as_secs_f32();
        let shift_multiplier = if self
            .metadata
            .keyboard_state
            .contains(&winit::keyboard::KeyCode::ShiftLeft)
        {
            5.0
        } else {
            1.0
        };

        let mut direction = glam::Vec3::ZERO;
        if self
            .metadata
            .keyboard_state
            .contains(&winit::keyboard::KeyCode::KeyW)
        {
            direction += self.camera.forward();
        }
        if self
            .metadata
            .keyboard_state
            .contains(&winit::keyboard::KeyCode::KeyS)
        {
            direction -= self.camera.forward();
        }
        if self
            .metadata
            .keyboard_state
            .contains(&winit::keyboard::KeyCode::KeyA)
        {
            direction -= self.camera.right();
        }
        if self
            .metadata
            .keyboard_state
            .contains(&winit::keyboard::KeyCode::KeyD)
        {
            direction += self.camera.right();
        }
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        self.camera.position += direction * self.camera.speed * delta_time * shift_multiplier;
        self.camera.update_rotation(self.metadata.delta_mouse);
        self.metadata.delta_mouse = Vec2::ZERO;
    }

    pub fn draw(&mut self) {
        let now = Instant::now();

        self.globals.frame += 1;
        self.globals.time_passed = (now - self.metadata.start_instant).as_secs_f32();
        self.globals.frame_time = (now - self.metadata.prev_frame_start_insant).as_secs_f32();
        self.metadata.prev_frame_start_insant = now;

        self.view = self.camera.get_view();
        self.update_uniforms();

        let mut frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture.");

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        self.run_cs(&mut encoder);
        self.run_rs(&mut encoder, &mut frame);

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
