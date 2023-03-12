// use wgpu::{Adapter, Color};
// use winit::{
//     event_loop::EventLoopBuilder, platform::windows::EventLoopBuilderExtWindows,
//     window::WindowBuilder,
// };

// fn main() {
//     println!("trivial example");
//     let event_loop = EventLoopBuilder::new()
//         .with_any_thread(false) // for now it's better for the event loop to be part of the main thread as it makes dispatch of events easier to handle
//         // .with_msg_hook(callback) // win32 msg callback, useful when setting up hooks
//         // and/or customized window theming related things, note this is dispatched async
//         // but is not really multi-threaded
//         .with_dpi_aware(false) // not needed!
//         .build();
//     let window = WindowBuilder::new()
//         .with_decorations(false)
//         .with_title("example trivial")
//         .with_maximized(false)
//         .build(&event_loop)
//         .expect("failed to create window!");

//     //  wgpu code
//     let instance = wgpu::Instance::default();
//     let surface =
//         unsafe { instance.create_surface(&window) }.expect("failed to create a wgpu surface!");
//     // it returns a future interesting?
//     let adapter_fut = instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
//         power_preference: wgpu::PowerPreference::HighPerformance,
//         force_fallback_adapter: false,
//         compatible_surface: Some(&surface),
//     });
//     let adapter = pollster::block_on(adapter_fut).expect("failed to request an adapter!");
//     let (device, queue) = pollster::block_on(adapter.request_device(
//         &wgpu::DeviceDescriptor {
//             features: wgpu::Features::empty(),
//             limits: wgpu::Limits::downlevel_webgl2_defaults(),
//             label: None,
//         },
//         None,
//     ))
//     .expect("failed to create device!");

//     // let viewport = ViewportDesc::new(window, Color::GREEN, &instance);
//     let caps = surface.get_capabilities(&adapter);
//     let size = window.inner_size();
//     let config = wgpu::SurfaceConfiguration {
//         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//         format: caps.formats[0],
//         width: size.width,
//         height: size.height,
//         present_mode: wgpu::PresentMode::Fifo,
//         alpha_mode: caps.alpha_modes[0],
//         view_formats: vec![],
//     };
//     surface.configure(&device, &config);

//     event_loop.run(move |event, _window_target, cf| {
//         match event {
//             // I assume this is the event loop key
//             // winit::event::Event::UserEvent(_) => {}
//             // winit::event::Event::NewEvents(_) => todo!(),
//             winit::event::Event::WindowEvent { event, .. } => {
//                 if let winit::event::WindowEvent::CloseRequested = event {
//                     // window_target.set_should_close(true); // there is a function with the exact same name in
//                     // GLFW not sure if the AI is using that for the inference here!? :/
//                     *cf = winit::event_loop::ControlFlow::Exit; // this should exit out of the event loop
//                 } else {
//                     *cf = winit::event_loop::ControlFlow::Wait; // this should continue the event loop
//                 }
//                 println!("window event: {event:?}");
//             }
//             winit::event::Event::DeviceEvent { event, .. } => {
//                 // println!("{device_id:?} + {event:?}");
//                 if let winit::event::DeviceEvent::Key(keyboard_input) = event {
//                     match keyboard_input.virtual_keycode {
//                         Some(winit::event::VirtualKeyCode::Escape) => {
//                             // alright pretty good job ai
//                             *cf = winit::event_loop::ControlFlow::Exit; // this should exit out of the event loop
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//             winit::event::Event::RedrawRequested(_) => {
//                 // we should redraw here if I am not wrong
//                 println!("redraw requested");
//                 let frame = surface
//                     .get_current_texture()
//                     .expect("failed to get current frame");
//                 let view = frame
//                     .texture
//                     .create_view(&wgpu::TextureViewDescriptor::default());
//                 let mut encoder =
//                     device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
//                 {
//                     let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                         label: None,
//                         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                             view: &view,
//                             resolve_target: None,
//                             ops: wgpu::Operations {
//                                 load: wgpu::LoadOp::Clear(Color::GREEN),
//                                 store: true,
//                             },
//                         })],
//                         depth_stencil_attachment: None,
//                     });
//                 }

//                 queue.submit(Some(encoder.finish()));
//                 frame.present();
//                 *cf = winit::event_loop::ControlFlow::Wait; // this should continue the event loop
//             }
//             // winit::event::Event::Suspended => todo!(),
//             // winit::event::Event::Resumed => todo!(),
//             // winit::event::Event::MainEventsCleared => todo!(),
//             // winit::event::Event::RedrawEventsCleared => todo!(),
//             // winit::event::Event::LoopDestroyed => todo!(),
//             _ => {
//                 *cf = winit::event_loop::ControlFlow::Wait; // this should continue the event loop
//             }
//         }
//     });
// }

use anyhow::Context;
use sgfx::{self, renderer::RectBound, shader::Shader};

// WE GOT TO OUR FIRST TRIANGLE with a WEBGPU Renderer!
// ----------------------------------------------------

fn main() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = sgfx::window::WindowBuilder::default()
        .set_title("my window")
        .set_fullscreen(true)
        .set_pos(0, 0)
        .build(&event_loop)
        .unwrap();
    let mut renderer =
        sgfx::renderer::Renderer::new(&window).context("failed to create renderer")?;
    event_loop.run(move |event, _, cf| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *cf = winit::event_loop::ControlFlow::Exit;
                    // ensure the cf is not changed later
                    return;
                }
                _ => {}
            },
            // sadge, guards don't allow for let bindings :P
            winit::event::Event::RedrawRequested(_wid) => {
                let shader = Shader::new(&renderer, r##"
                @vertex
                fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                    let x = f32(i32(in_vertex_index) - 1);
                    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
                    return vec4<f32>(x, y, 0.0, 1.0);
                }

                @fragment
                fn fs_main() -> @location(0) vec4<f32> {
                    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
                }
                "##);
                if let Ok(mut render_frame) = renderer.get_render_frame() {
                    const MAGENTA: wgpu::Color = wgpu::Color {
                        r: 1.0,
                        g: 0.0,
                        b: 1.0,
                        a: 0.0,
                    };
                    const GREEN: wgpu::Color = wgpu::Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 0.0,
                    };
                    render_frame.clear_screen(MAGENTA);
                    render_frame.draw_rect(shader, GREEN, RectBound{ left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 });
                    // completely not sure if this is the best way to do this
                    // LOL AI is too good at reading my mind
                    // _ =  render_frame.draw_rect(
                    //     &wgpu::Rect {
                    //         left: 0.0,
                    //         right: 1.0,
                    //         top: 0.0,
                    //         bottom: 1.0,
                    //     }
                    // );

                    render_frame.render();
                }
            }
            _ => {}
        }
        *cf = winit::event_loop::ControlFlow::Wait;
    });
}
