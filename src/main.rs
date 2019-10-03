#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use crate::instructions::InstructionParser;
use std::env;

mod bitmasks;
mod core;
mod display;
mod instructions;
mod opcodes;
// mod opcodesv2;
// mod ophandlers;

fn main() {
    env_logger::init();
    let rom_file = env::args().nth(1).expect("Please input a ROM file");
    let ins_parser = opcodes::OpcodeMaskParser {};
    let mut vm = core::Machine::new("Chip8", ins_parser);
    vm.load_rom(&rom_file)
        .expect("Unable to load ROM from file");
    debug!("{:#?}", vm);
    vm.start().unwrap();
}
