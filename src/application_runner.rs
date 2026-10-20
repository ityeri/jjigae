use shared::crevice::std140::{AsStd140, Std140};
use std::sync::Arc;
use std::time::Instant;
use wgpu::{BufferDescriptor, Features};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::application_handler::JjigaeApplicationHandler;

pub struct JjigaeApplicationRunner<H>
where
    H: JjigaeApplicationHandler,
{
    handler: H,
    render_context: Option<RenderContext>,
}

pub struct RenderContext {
    pub window: Arc<Window>,
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub pipeline: wgpu::RenderPipeline,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub start: Instant,
}

impl<H> JjigaeApplicationRunner<H>
where
    H: JjigaeApplicationHandler,
{
    pub fn new(app_handler: H) -> Self {
        JjigaeApplicationRunner {
            handler: app_handler,
            render_context: None,
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::<()>::with_user_event().build().unwrap();
        event_loop.run_app(self).unwrap();
    }

    async fn init_render_context(&self, window: Arc<Window>) -> anyhow::Result<RenderContext> {
        let size = window.inner_size();

        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());

        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("device"),
                required_features: Features::SHADER_I16,
                ..Default::default()
            })
            .await?;

        let mut surface_config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .ok_or_else(|| anyhow::anyhow!("surface not supported by adapter"))?;
        surface_config.present_mode = wgpu::PresentMode::Fifo;
        surface.configure(&device, &surface_config);

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::util::make_spirv(self.handler.get_shader()),
        });

        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("uniform"),
            size: H::BufferBodyType::std140_size_static() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("params_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("params_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        // let pipeline = device.create_render_pipeline(&self.handler.create_render_pipeline(
        //     &pipeline_layout,
        //     &shader_module,
        //     &surface_config,
        // ));
        let pipeline = self.handler.create_render_pipeline(
            &device,
            &pipeline_layout,
            &shader_module,
            &surface_config,
        );

        Ok(RenderContext {
            window,
            surface,
            device,
            queue,
            config: surface_config,
            size,
            pipeline,
            uniform_buffer,
            bind_group,
            start: Instant::now(),
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        match &mut self.render_context {
            Some(render_context) => {
                if width > 0 && height > 0 {
                    render_context.size = winit::dpi::PhysicalSize::new(width, height);
                    render_context.config.width = width;
                    render_context.config.height = height;
                    render_context
                        .surface
                        .configure(&render_context.device, &render_context.config);
                }

                self.handler.on_resize(width, height);
            }
            None => {}
        }
    }

    fn render(&mut self) {
        let render_context = match &self.render_context {
            Some(render_context) => render_context,
            None => return,
        };

        let buffer_value = self.handler.get_uniform_buffer_value();
        render_context.queue.write_buffer(
            &render_context.uniform_buffer,
            0,
            &buffer_value.as_std140().as_bytes(),
        );

        let output = match render_context.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t)
            | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            // Surface is to old or resized (changed)
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                let (w, h) = (render_context.size.width, render_context.size.height);
                self.resize(w, h);
                return;
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => return,
            wgpu::CurrentSurfaceTexture::Validation => {
                eprintln!("surface validation error");
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            render_context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("encoder"),
                });

        {
            // let mut render_pass = encoder.begin_render_pass(&self.handler.gen_render_pass(&view));
            let mut render_pass = self.handler.begin_render_pass(&mut encoder, &view);
            render_pass.set_pipeline(&render_context.pipeline);
            render_pass.set_bind_group(0, &render_context.bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        render_context
            .queue
            .submit(std::iter::once(encoder.finish()));
        render_context.queue.present(output);
    }
}

impl<H> ApplicationHandler for JjigaeApplicationRunner<H>
where
    H: JjigaeApplicationHandler,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.render_context =
            Some(pollster::block_on(self.init_render_context(window.clone())).unwrap());

        self.handler
            .on_resumed(&self.render_context.as_ref().unwrap());

        // The start of first frame
        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                self.handler.on_tick();
                self.render();

                match &self.render_context {
                    Some(render_context) => {
                        render_context.window.request_redraw();
                    }
                    None => return,
                };
            }
            _ => {}
        }
        self.handler.on_window_event(event, event_loop);
    }
}
