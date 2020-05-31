use std::io;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Args {
    /// Start with debugging info
    #[clap(short = "D", long)]
    debug: bool,
    /// The rom to use
    rom: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("error: {}", e);
    }
}

fn run(args: Args) -> io::Result<()> {
    todo!()
}
