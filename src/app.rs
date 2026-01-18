use std::sync::Arc;

use crate::graphics::graphics::*;
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::{DeviceEvent, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

enum State {
    Ready(Graphics),
    Init(Option<EventLoopProxy<Graphics>>),
}

pub struct App {
    state: State,
}

impl App {
    pub fn new(event_loop: &EventLoop<Graphics>) -> Self {
        Self {
            state: State::Init(Some(event_loop.create_proxy())),
        }
    }

    fn update(&mut self) {
        if let State::Ready(gfx) = &mut self.state {
            gfx.update();
        }
    }

    fn draw(&mut self) {
        if let State::Ready(gfx) = &mut self.state {
            gfx.draw();
            gfx.request_redraw();
        }
    }

    fn resized(&mut self, size: PhysicalSize<u32>) {
        if let State::Ready(gfx) = &mut self.state {
            gfx.resize(size);
        }
    }

    fn cursor_moved(&mut self, position: &PhysicalPosition<f64>) {
        if let State::Ready(gfx) = &mut self.state {
            gfx.set_mouse_pos([position.x as f32, position.y as f32].into());
        }
    }

    fn mouse_motion(&mut self, delta_x: f32, delta_y: f32) {
        if let State::Ready(gfx) = &mut self.state {
            gfx.handle_mouse_motion(delta_x, delta_y);
        }
    }

    fn capture_mouse(&mut self) {
        if let State::Ready(gfx) = &mut self.state {
            let _ = gfx
                .window
                .set_cursor_grab(winit::window::CursorGrabMode::Locked)
                .or_else(|_| {
                    gfx.window
                        .set_cursor_grab(winit::window::CursorGrabMode::Confined)
                })
                .expect("Failed to grab cursor");

            gfx.window.set_cursor_visible(false);
        }
    }

    fn release_mouse(&mut self) {
        if let State::Ready(gfx) = &mut self.state {
            let _ = gfx
                .window
                .set_cursor_grab(winit::window::CursorGrabMode::None)
                .expect("Failed to release cursor");

            gfx.window.set_cursor_visible(true);
        }
    }

    fn keyboard_input(&mut self, event: &KeyEvent) {
        if let State::Ready(gfx) = &mut self.state {
            gfx.handle_keyboard_input(event);
        }
    }
}

impl ApplicationHandler<Graphics> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let State::Init(proxy) = &mut self.state {
            if let Some(proxy) = proxy.take() {
                let mut win_attr = Window::default_attributes();

                win_attr = win_attr.with_title("WebGPU example");

                let window = Arc::new(
                    event_loop
                        .create_window(win_attr)
                        .expect("create window err."),
                );

                pollster::block_on(create_graphics(window, proxy));
            }
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, gfx: Graphics) {
        gfx.request_redraw();
        self.state = State::Ready(gfx);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => self.resized(size),
            WindowEvent::RedrawRequested => {
                self.update();
                self.draw();
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => self.cursor_moved(&position),
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                if button == winit::event::MouseButton::Left
                    && state == winit::event::ElementState::Pressed
                {
                    self.capture_mouse();
                }

                if button == winit::event::MouseButton::Right
                    && state == winit::event::ElementState::Pressed
                {
                    self.release_mouse();
                }
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if event.physical_key == PhysicalKey::Code(winit::keyboard::KeyCode::Escape)
                    && event.state == winit::event::ElementState::Pressed
                {
                    self.release_mouse();
                }
                self.keyboard_input(&event);
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                let (dx, dy) = delta;
                self.mouse_motion(dx as f32, dy as f32);
            }
            _ => {}
        }
    }
}
