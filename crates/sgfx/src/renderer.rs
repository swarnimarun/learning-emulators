use anyhow::Context;
use wgpu::{CommandBuffer, TextureView};

use crate::{
    shader::{self, Shader},
    window::Window,
};

pub struct RenderFrame<'a> {
    renderer: &'a mut Renderer,
    frame: wgpu::SurfaceTexture,
    commands: Vec<RenderCommand>,
}

impl RenderFrame<'_> {
    pub fn clear_screen(&mut self, color: wgpu::Color) {
        self.commands.push(RenderCommand::ClearScreen(color));
    }
    pub fn draw_rect(&mut self, shader: Shader, color: wgpu::Color, rect: RectBound) {
        self.commands.push(RenderCommand::Rect(shader, color, rect));
    }
    pub fn render(self) {
        let view = self
            .frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let render_buffer: RenderCommandBuffer = self.commands.into();
        self.renderer
            .queue
            .push(render_buffer.build_command_buffer(&self.renderer, &view));
        self.frame.present();
    }
}

pub struct RectBound {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

pub enum RenderCommand {
    // command to draw a rect
    Rect(Shader, wgpu::Color, RectBound),
    // simple clear screen command, to simplify
    // the webgpu api before using it in neo8
    ClearScreen(wgpu::Color),
}

pub struct RenderCommandBuffer {
    pub commands: Vec<RenderCommand>,
}

impl From<Vec<RenderCommand>> for RenderCommandBuffer {
    fn from(commands: Vec<RenderCommand>) -> Self {
        Self { commands }
    }
}

impl RenderCommandBuffer {
    pub fn build_command_buffer(
        self,
        renderer: &Renderer,
        view: &TextureView,
    ) -> wgpu::CommandBuffer {
        let mut encoder = renderer
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let mut clear_color = wgpu::Color::BLACK;
        let mut render_pipeline = None;
        for command in self.commands {
            match command {
                RenderCommand::ClearScreen(color) => {
                    // add a render pass for clean screen
                    clear_color = color;
                }
                RenderCommand::Rect(shader, color, rect) => {
                    // for now we only draw a triangle
                    let layout =
                        renderer
                            .device
                            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                                label: None,
                                bind_group_layouts: &[],
                                push_constant_ranges: &[],
                            });
                    render_pipeline = Some(renderer.device.create_render_pipeline(
                        &wgpu::RenderPipelineDescriptor {
                            label: None,
                            layout: Some(&layout),
                            vertex: wgpu::VertexState {
                                module: &shader.shader_module,
                                entry_point: "vs_main",
                                buffers: &[],
                            },
                            primitive: wgpu::PrimitiveState::default(),
                            depth_stencil: None,
                            multisample: wgpu::MultisampleState::default(),
                            fragment: Some(wgpu::FragmentState {
                                module: &shader.shader_module,
                                entry_point: "fs_main",
                                // this is apparently very important lol
                                targets: &[Some(renderer.capabilities.formats[0].into())],
                            }),
                            multiview: None,
                        },
                    ));
                } // _ => {
                  //     unimplemented!("the command support hasn't been implemented yet!")
                  // }
            }
        }
        {
            if let Some(render_pipeline) = render_pipeline {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
                render_pass.set_pipeline(&render_pipeline);
                // this is the vertex buffer indexes to draw in order
                // exactly a triangle
                render_pass.draw(0..3, 0..1);
            } else {
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
            }
        }
        // render_pass.set_pipeline(&self.build_pipeline(device));
        encoder.finish()
    }
}

#[derive(Debug)]
pub struct RenderQueue(wgpu::Queue);

impl RenderQueue {
    pub fn push(&mut self, cbuf: CommandBuffer) -> bool {
        self.0.submit(Some(cbuf));
        false
    }
}

#[derive(Debug)]
pub struct Renderer {
    _instance: wgpu::Instance,
    _adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    surface: wgpu::Surface,
    _config: wgpu::SurfaceConfiguration,
    queue: RenderQueue,
    capabilities: wgpu::SurfaceCapabilities,
}

impl Renderer {
    pub fn new(window: &Window) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window.winit_window) }
            .context("failed to create wgpu surface")?;
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .context("failed to request an adapter")?;
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: None,
            },
            None,
        ))
        .context("failed to create device")?;
        let capabilities = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width: window.winit_window.inner_size().width,
            height: window.winit_window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        Ok(Self {
            _instance: instance,
            _adapter: adapter,
            device,
            surface,
            queue: RenderQueue(queue),
            _config: config,
            capabilities,
        })
    }
    /// we need to create a render frame to render to the screen
    /// this is a wgpu::SurfaceTexture
    /// with the renderer attached as processor
    /// -----------------------------------------------------------------
    /// this is a separate struct as that makes handling lifetimes easier
    pub fn get_render_frame(&mut self) -> anyhow::Result<RenderFrame> {
        let frame = self
            .surface
            .get_current_texture()
            .context("failed to get current frame")?;
        Ok(RenderFrame {
            renderer: self,
            frame,
            commands: vec![],
        })
    }
}
