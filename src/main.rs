#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use std::env;

mod audio;
mod bitmasks;
mod core;
mod display;
mod instructions;
mod keyboard;
mod opcodes;

fn main() {
    env_logger::init();
    let rom_file = env::args().nth(1).expect("Please input a ROM file");
    let ins_parser = opcodes::OpcodeMaskParser {};
    let sdl_context = sdl2::init().unwrap();
    let mut vm = core::Machine::new("Chip8", ins_parser, false, Some(sdl_context));
    vm.load_rom(&rom_file)
        .expect("Unable to load ROM from file");
    debug!("{:#?}", vm);
    vm.start().unwrap();
}
