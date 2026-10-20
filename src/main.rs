use glam::{Vec2, vec2};
use jjigae::{
    application_handler::JjigaeApplicationHandler,
    application_runner::{JjigaeApplicationRunner, RenderContext},
};
use plock::PlockClock;
use scrollrs::{Projector, ScrollAdapter};
use shared::{GpuCamera, Parameter, shared_glam};
use wgpu::{
    CommandEncoder, RenderPass, RenderPassDescriptor, RenderPipelineDescriptor, ShaderModule,
    SurfaceConfiguration, TextureView,
};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent},
};

const SHADER: &[u8] = include_bytes!(env!("shader.spv"));

struct App {
    screen_size: PhysicalSize<u32>,
    clock: PlockClock,
    scroll_adapter: ScrollAdapter,
    current_mouse_pos: Vec2,
    last_mouse_pos: Vec2,
    current_mouse_pressed: bool,
}
impl App {
    pub fn default() -> Self {
        App {
            screen_size: PhysicalSize::new(0, 0),
            clock: PlockClock::new(1.0 / 60.0),
            scroll_adapter: ScrollAdapter::new(500.0, 500.0, None, None, None),
            current_mouse_pos: Vec2::new(250.0, 250.0),
            last_mouse_pos: Vec2::new(250.0, 250.0),
            current_mouse_pressed: false,
        }
    }
}

impl JjigaeApplicationHandler for App {
    type BufferBodyType = Parameter;

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
                entry_point: Some("main_vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader_module,
                entry_point: Some("main_fs"),
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
        self.scroll_adapter = self.scroll_adapter.resized(
            self.screen_size.width as f64,
            self.screen_size.height as f64,
            None,
            None,
        );
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
                        self.scroll_adapter = self.scroll_adapter.zoom(zoom_amount);
                    }
                    MouseScrollDelta::LineDelta { 0: _h, 1: v } => {
                        let zoom_amount = v as f64 * 0.1;
                        self.scroll_adapter = self.scroll_adapter.zoom(zoom_amount);
                    }
                },
                _ => {}
            },
            _ => {}
        }
    }

    fn on_tick(&mut self) {
        self.scroll_adapter = self.scroll_adapter.updated(self.clock.last_dt);

        if self.current_mouse_pressed {
            let mouse_delta = -(self.current_mouse_pos - self.last_mouse_pos).as_dvec2();
            self.scroll_adapter
                .moved(self.scroll_adapter.unproject_scalev(mouse_delta));
        }

        self.clock = self.clock.tick(-1.0);
        self.last_mouse_pos = self.current_mouse_pos;
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        self.screen_size = PhysicalSize::new(width, height);
        self.scroll_adapter = self.scroll_adapter.resized(
            self.screen_size.width as f64,
            self.screen_size.height as f64,
            None,
            None,
        );
    }

    fn get_uniform_buffer_value(&self) -> Self::BufferBodyType {
        Parameter {
            resolution: shared_glam::vec2(
                self.screen_size.width as f32,
                self.screen_size.height as f32,
            ),
            camera: GpuCamera {
                position: shared_glam::vec2(
                    self.scroll_adapter.camera.position.x as f32,
                    self.scroll_adapter.camera.position.y as f32,
                ),
                zoom: self.scroll_adapter.camera.zoom as f32,
            },
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

fn main() {
    JjigaeApplicationRunner::new(App::default()).run();
}
