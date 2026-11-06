use std::f32::consts::PI;

use crate::{
    application_handler::JjigaeApplicationHandler,
    application_runner::{JjigaeApplicationRunner, RenderContext},
};
use glam::{Vec2, Vec3, vec2, vec3};
use plock::PlockClock;
use shared::{
    camera3::{Camera3, Viewport3},
    shared_glam,
};
use wgpu::{
    CommandEncoder, RenderPass, RenderPassDescriptor, RenderPipelineDescriptor, ShaderModule,
    SurfaceConfiguration, TextureView,
};
use winit::{
    dpi::PhysicalSize,
    event::{self, ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

const SHADER: &[u8] = include_bytes!(env!("shader.spv"));

pub struct Viewport3App {
    screen_size: PhysicalSize<u32>,
    clock: PlockClock,
    current_mouse_pos: Vec2,
    last_mouse_pos: Vec2,
    current_mouse_pressed: bool,
    key_state: KeyState,
    camera: Camera3,
}
impl Viewport3App {
    pub fn default() -> Self {
        Viewport3App {
            screen_size: PhysicalSize::new(0, 0),
            clock: PlockClock::new(1.0 / 60.0),
            current_mouse_pos: Vec2::new(250.0, 250.0),
            last_mouse_pos: Vec2::new(250.0, 250.0),
            current_mouse_pressed: false,
            key_state: KeyState::new(),
            camera: Camera3 {
                position: shared_glam::vec3(0.0, 0.0, -2.0),
                pitch: 0.0,
                yaw: 0.0,
                pov: 60.0f32.to_radians(),
            },
        }
    }
}

#[derive(Clone)]
struct KeyState {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}
impl KeyState {
    pub fn new() -> Self {
        KeyState {
            forward: false,
            backward: false,
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }
    pub fn on_event(&self, event: KeyEvent) -> Self {
        let is_pressing = match event.state {
            ElementState::Pressed => true,
            ElementState::Released => false,
        };

        match event.physical_key {
            PhysicalKey::Code { 0: code } => match code {
                KeyCode::KeyW => KeyState {
                    forward: is_pressing,
                    ..self.clone()
                },
                KeyCode::KeyS => KeyState {
                    backward: is_pressing,
                    ..self.clone()
                },
                KeyCode::KeyA => KeyState {
                    left: is_pressing,
                    ..self.clone()
                },
                KeyCode::KeyD => KeyState {
                    right: is_pressing,
                    ..self.clone()
                },
                KeyCode::Space => KeyState {
                    up: is_pressing,
                    ..self.clone()
                },
                KeyCode::ShiftLeft => KeyState {
                    down: is_pressing,
                    ..self.clone()
                },
                _ => self.clone(),
            },
            _ => self.clone(),
        }
    }
}

impl JjigaeApplicationHandler for Viewport3App {
    type BufferBodyType = Viewport3;

    fn get_shader(&self) -> &[u8] {
        SHADER
    }

    fn create_render_pipeline(
        &self,
        device: &wgpu::Device,
        pipeline_layout: &wgpu::PipelineLayout,
        shader_module: &ShaderModule,
        surface_config: &SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader_module,
                entry_point: Some("fullscreen_vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader_module,
                entry_point: Some("flatcube_shader::main_fs"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        })
    }

    fn on_resumed(&mut self, render_context: &RenderContext) {
        self.screen_size = render_context.window.inner_size();
    }

    fn on_window_event(
        &mut self,
        event: WindowEvent,
        event_loop: &winit::event_loop::ActiveEventLoop,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.key_state = self.key_state.on_event(event);
            }
            WindowEvent::MouseInput {
                state: button_state,
                button,
                ..
            } => match button {
                MouseButton::Left => match button_state {
                    ElementState::Pressed => {
                        self.current_mouse_pressed = true;
                    }
                    ElementState::Released => {
                        self.current_mouse_pressed = false;
                    }
                },
                _ => {}
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.current_mouse_pos = vec2(position.x as f32, position.y as f32);
            }
            WindowEvent::MouseWheel { delta, phase, .. } => match phase {
                TouchPhase::Moved => match delta {
                    MouseScrollDelta::PixelDelta { 0: delta } => {
                        let zoom_amount = delta.y as f64 * 0.01;
                    }
                    MouseScrollDelta::LineDelta { 0: _h, 1: v } => {
                        let zoom_amount = v as f64 * 0.1;
                    }
                },
                _ => {}
            },
            _ => {}
        }
    }

    fn on_tick(&mut self) {
        if self.current_mouse_pressed {
            let mouse_delta = (self.current_mouse_pos - self.last_mouse_pos).as_dvec2();

            self.camera.yaw += mouse_delta.x as f32 * 0.003;
            self.camera.pitch += mouse_delta.y as f32 * 0.003;

            self.camera.pitch = if self.camera.pitch < -PI / 2.0 {
                -PI / 2.0
            } else if PI / 2.0 < self.camera.pitch {
                PI / 2.0
            } else {
                self.camera.pitch
            }
        }

        let position_delta = vec![
            if self.key_state.forward {
                Vec3::Z
            } else {
                Vec3::ZERO
            },
            if self.key_state.backward {
                Vec3::NEG_Z
            } else {
                Vec3::ZERO
            },
            if self.key_state.left {
                Vec3::NEG_X
            } else {
                Vec3::ZERO
            },
            if self.key_state.right {
                Vec3::X
            } else {
                Vec3::ZERO
            },
            if self.key_state.up {
                Vec3::Y
            } else {
                Vec3::ZERO
            },
            if self.key_state.down {
                Vec3::NEG_Y
            } else {
                Vec3::ZERO
            },
        ]
        .iter()
        .sum::<Vec3>()
            * 0.003;

        self.camera.position += {
            let world_delta = position_delta.rotate_y(self.camera.yaw);
            shared_glam::vec3(world_delta.x, world_delta.y, world_delta.z)
        };

        self.clock = self.clock.tick(-1.0);
        self.last_mouse_pos = self.current_mouse_pos;
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        self.screen_size = PhysicalSize::new(width, height);
    }

    fn get_uniform_buffer_value(&self) -> Self::BufferBodyType {
        Viewport3 {
            resolution: shared_glam::vec2(
                self.screen_size.width as f32,
                self.screen_size.height as f32,
            ),
            camera: self.camera.clone(),
        }
    }

    fn begin_render_pass<'a>(
        &self,
        encoder: &'a mut CommandEncoder,
        view: &'a TextureView,
    ) -> RenderPass<'a> {
        encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }
}
