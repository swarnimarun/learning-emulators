use clap::{Parser, Subcommand};
use color_eyre::Result;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
struct App {
    #[clap(subcommand)]
    subcommands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Nes(res::App),
    Chip8(chiprs::App),
    Gameboy(gamebors::App),
}

pub fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_ansi(true)
        .with_env_filter({
            let env_builder = EnvFilter::builder();
            #[cfg(debug_assertions)]
            let env_builder = env_builder.with_default_directive(LevelFilter::TRACE.into());
            env_builder.from_env_lossy()
        })
        .init();

    info!(target: "function");

    let app = App::parse();
    match app.subcommands {
        Commands::Nes(nes) => nes.start(),
        Commands::Chip8(mut chip8) => {
            chip8.init()?;
            chip8.run();
        }
        Commands::Gameboy(gb) => gb.start(),
    }
    Ok(())
}
