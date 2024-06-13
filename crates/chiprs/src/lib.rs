mod core;
use core::{processor::RendererState, Processor, Rom};
use std::{collections::HashMap, path::PathBuf, time::Instant};

use clap::Parser;
use color_eyre::Result;

use tracing::{info, instrument, warn};
use winit::{
    application::ApplicationHandler, event::KeyEvent, keyboard::KeyCode, window::WindowAttributes,
};

#[derive(Parser, Debug)]
pub struct App {
    pub rom: PathBuf,
    #[arg(short, long, alias = "di", default_value_t = false)]
    pub disassemble: bool,
    #[clap(skip)]
    processor: Option<Processor>,
    #[clap(skip)]
    window: Option<winit::window::Window>,
    #[clap(skip)]
    events: EventMap,
    #[clap(skip)]
    state: RendererState,
}

type EventMap = HashMap<KeyCode, KeyEvent>;

impl App {
    #[instrument]
    pub fn disassemble_rom(self) -> Result<()> {
        info!("disassembling the rom, {}", self.rom.display());
        Rom::load_from_path(&self.rom)?.rom_disassemble()
    }

    #[instrument]
    pub fn init(&mut self) -> Result<()> {
        info!(
            "initializing chiprs processor with rom, {}",
            self.rom.display()
        );
        _ = self
            .processor
            .insert(Processor::with_rom(Rom::load_from_path(&self.rom)?));
        // ensure the instant is updated before hand
        _ = self.state.instant.insert(Instant::now());
        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            if let Ok(w) = event_loop.create_window(
                WindowAttributes::default()
                    .with_title("chiprs app")
                    .with_active(true)
                    .with_visible(true)
                    .with_resizable(true),
            ) {
                info!("window created: {:?}", w.id());
                _ = self.window.insert(w);
            }
        } else {
            warn!("window already exists");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Self {
            rom: _,
            processor,
            window,
            events: _,
            state,
            ..
        } = self;

        let Some(window) = window else {
            return;
        };
        match event {
            winit::event::WindowEvent::Resized(size) => {
                info!("resized window: {size:?}");
                window.request_redraw();
                // resize with pixel buffer
                if let Some(Processor {
                    framebuffer: Some(fb),
                    ..
                }) = processor
                {
                    fb.resize(size.width, size.height);
                }
            }
            winit::event::WindowEvent::CloseRequested => {}
            winit::event::WindowEvent::RedrawRequested => {
                // render function with state
                if let Some(proc) = processor {
                    _ = proc.run(&window, state);
                } else {
                    warn!("processor not initialized.");
                }
                // keep redrawing every frame
                window.request_redraw();
            }
            _ => {}
        }
    }
}
