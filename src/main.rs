use glam::{UVec2, uvec2};
use jjigae::{
    application_handler::JjigaeApplicationHandler,
    application_runner::{JjigaeApplicationRunner, RenderContext},
};
use shared::{GpuCamera, Parameter, shared_glam};
use wgpu::{
    CommandEncoder, RenderPass, RenderPassDescriptor, RenderPipelineDescriptor, ShaderModule,
    SurfaceConfiguration, TextureView,
};
use winit::{dpi::PhysicalSize, event_loop::EventLoop};

const SHADER: &[u8] = include_bytes!(env!("shader.spv"));

struct App {
    screen_size: PhysicalSize<u32>,
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
        self.screen_size = render_context.window.inner_size()
    }

    fn on_window_event(
        &mut self,
        event: winit::event::WindowEvent,
        event_loop: &winit::event_loop::ActiveEventLoop,
    ) {
    }

    fn on_tick(&mut self) {}

    fn on_resize(&mut self, width: u32, height: u32) {
        self.screen_size = PhysicalSize::new(width, height);
    }

    fn get_uniform_buffer_value(&self) -> Self::BufferBodyType {
        Parameter {
            resolution: shared_glam::vec2(
                self.screen_size.width as f32,
                self.screen_size.height as f32,
            ),
            camera: GpuCamera {
                position: shared_glam::vec2(0.0, 0.0),
                zoom: 0.1,
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
    JjigaeApplicationRunner::new(App {
        screen_size: PhysicalSize::new(0, 0),
    })
    .run();
}
