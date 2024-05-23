use std::{collections::HashMap, sync::Arc};

use color_eyre::{eyre::Context, Result};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;
use wgpu::{
    CommandEncoderDescriptor, RenderPassColorAttachment, RenderPassDescriptor,
    TextureViewDescriptor,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

struct WgpuContext {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    window: Arc<Window>,
    render_pipeline: Option<wgpu::RenderPipeline>,
}

impl WgpuContext {
    fn render(&mut self) -> Result<()> {
        let context = self;
        let surface_texture = context.surface.get_current_texture()?;

        let texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = context
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        // clear screen render pass
        for render_pipeline in &context.render_pipeline {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("clearscreen"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(render_pipeline);
            render_pass.draw(0..3, 0..2);
        }

        context.queue.submit([encoder.finish()]);
        surface_texture.present();

        Ok(())
    }
    pub fn new(window: Arc<Window>) -> Result<WgpuContext> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        // note: we can't drop window before surface
        let surface = instance.create_surface(window.clone())?;

        let (adapter, device, queue) = tokio::runtime::Runtime::new().unwrap().block_on(async {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .unwrap();
            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        required_features: wgpu::Features::empty(),
                        // WebGL doesn't support all of wgpu's features, so if
                        // we're building for the web, we'll have to disable some.
                        required_limits: if cfg!(target_arch = "wasm32") {
                            wgpu::Limits::downlevel_webgl2_defaults()
                        } else {
                            wgpu::Limits::default()
                        },
                        label: None,
                    },
                    None, // Trace path
                )
                .await
                .unwrap();
            (adapter, device, queue)
        });
        let surface_caps = surface.get_capabilities(&adapter);

        // we are only going to support sRGB surfaces for this example
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/simple.wgsl").into()),
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        //device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        //    label: Some("com"),
        //    layout: Some(&render_pipeline_layout),
        //    module: &shader,
        //    entry_point: "test",
        //    compilation_options: wgpu::PipelineCompilationOptions::default(),
        //});
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        Ok(WgpuContext {
            surface,
            device,
            queue,
            config,
            window,
            render_pipeline: Some(render_pipeline),
        })
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        info!("resized to {size:?}");
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        self.window.request_redraw();
    }
}

#[derive(Default)]
struct App {
    contexts: HashMap<WindowId, WgpuContext>,
    window_attrs: WindowAttributes,
}

impl App {
    fn with_window_attrs(window_attrs: WindowAttributes) -> Self {
        Self {
            window_attrs,
            ..Default::default()
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // create a window here
        if let Ok(window) = event_loop.create_window(self.window_attrs.clone()) {
            let wid = window.id();
            // get surface ready for rendering
            match WgpuContext::new(Arc::new(window)) {
                Ok(context) => self.contexts.insert(wid, context),
                Err(err) => {
                    tracing::error!("failed to create window context: {:?}", err);
                    return;
                }
            };
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => {
                let Some(context) = self.contexts.get_mut(&window_id) else {
                    return;
                };
                context.resize(size);
            }
            WindowEvent::CloseRequested => {
                // close / exit
                // drop window first
                self.contexts.remove(&window_id);
                // event loop exit if no windows
                if self.contexts.is_empty() {
                    event_loop.exit()
                }
            }
            WindowEvent::RedrawRequested => {
                // keep re-drawing
                let Some(context) = self.contexts.get_mut(&window_id) else {
                    return;
                };
                _ = context.render();
                context.window.request_redraw();
            }
            _ => {}
        }
    }
}

pub fn main() -> Result<()> {
    let env_filter = if cfg!(debug_assertions) {
        EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy()
    } else {
        EnvFilter::from_default_env()
    };
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let ev = EventLoop::new()?;
    ev.set_control_flow(ControlFlow::Poll);
    let mut app = App::with_window_attrs(
        WindowAttributes::default()
            .with_title("simple-example")
            .with_active(true),
    );
    ev.run_app(&mut app)
        .context("Failed to run the application successfully")
}
