mod core;
use core::{Processor, Rom};
use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;

use tracing::{info, instrument};

#[derive(Parser, Debug)]
pub struct App {
    rom: PathBuf,
    #[clap(skip)]
    processor: Option<Processor>,
}

impl App {
    #[instrument(name = "chiprs::App::init")]
    pub fn init(&mut self) -> Result<()> {
        _ = self
            .processor
            .insert(Processor::with_rom(Rom::load_from_path(&self.rom)?));
        Ok(())
    }
    pub fn run(&self) {
        if let Some(p) = &self.processor {
            // event loop start
        }
    }
}
