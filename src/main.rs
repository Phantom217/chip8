use std::io;
use std::path::PathBuf;

use clap::{AppSettings, Clap};

use chip8::Emulator;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Args {
    /// Start with debugging info
    #[clap(short = "D", long)]
    debug: bool,
    /// The rom to use
    #[clap(parse(from_os_str))]
    rom: PathBuf,
}

fn main() {
    let args = Args::parse();

    use chip8::opcode::OpCode;
    use chip8::types::Addr;
    let opcode = OpCode::from((0xFA, 0xCE));
    let opcode_tuple = opcode.to_match_tuple();
    let addr = Addr::from(0xFACE);
    println!("{}", addr);
    println!("{:#04X?}", opcode_tuple);

    println!("\n\n{}", opcode_tuple.1);

    use chip8::Chip8;
    let mut emu = Chip8::new();
    emu.load_rom(&args.rom).unwrap();

    println!("{:?}", emu.ram)

    // if let Err(e) = run(args) {
    //     eprintln!("error: {}", e);
    // }
}

fn run(args: Args) -> io::Result<()> {
    todo!()
}
