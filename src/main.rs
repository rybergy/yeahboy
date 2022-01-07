#[macro_use]
extern crate lazy_static;

use simple_logger::SimpleLogger;
use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
};
use structopt::StructOpt;

mod bits;
mod cpu;
mod debugger;
mod mmu;
mod instructions;

/// A gameboy emulator.
///
/// Pls work
#[derive(Debug, StructOpt)]
#[structopt(
    name = "yeahboy",
    about = "Yeeeaah boyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"
)]
struct Opt {
    /// Path to the ROM.
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
}

fn load_rom<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    SimpleLogger::new().with_utc_timestamps().init().unwrap();

    let rom = load_rom(opt.rom)?;

    log::warn!("test");

    debugger::run(rom);

    Ok(())
}
