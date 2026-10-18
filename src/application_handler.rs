use shared::crevice::std140::AsStd140;
use wgpu::{
    CommandEncoder, Device, PipelineLayout, RenderPass, RenderPipeline, ShaderModule,
    SurfaceConfiguration, TextureView,
};
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

use crate::application_runner::RenderContext;

pub trait JjigaeApplicationHandler {
    type BufferBodyType: AsStd140;

    fn get_shader(&self) -> &[u8];
    fn create_render_pipeline(
        &self,
        device: &Device,
        pipeline_layout: &PipelineLayout,
        shader_module: &ShaderModule,
        surface_config: &SurfaceConfiguration,
    ) -> RenderPipeline;

    fn on_resumed(&mut self, render_context: &RenderContext);
    fn on_window_event(&mut self, event: WindowEvent, event_loop: &ActiveEventLoop);
    fn on_tick(&mut self);
    fn on_resize(&mut self, width: u32, height: u32);

    fn get_uniform_buffer_value(&self) -> Self::BufferBodyType;
    fn begin_render_pass<'a>(
        &self,
        encoder: &'a mut CommandEncoder,
        view: &'a TextureView,
    ) -> RenderPass<'a>;
}
