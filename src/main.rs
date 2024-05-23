use clap::{Parser, Subcommand};

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

pub fn main() {
    let app = App::parse();
    match app.subcommands {
        Commands::Nes(nes) => nes.start(),
        Commands::Chip8(chip8) => chip8.start(),
        Commands::Gameboy(gb) => gb.start(),
    }
}
