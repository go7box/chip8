#[macro_use]
extern crate log;
extern crate env_logger;

use crate::instructions::InstructionParser;
use std::env;
use std::thread::JoinHandle;

mod bitmasks;
mod core;
mod instructions;
mod opcodes;
mod opcodesv2;
mod ophandlers;

/**
 * Start the machine in a separate thread.
 * We do this because we need to be able to parse instructions in one
 * thread and render the output in another. Otherwise we will block on
 * each instruction while doing the rendering.
*/
pub fn launch_thread<T>(
    mut machine: core::Machine<T>,
) -> JoinHandle<std::result::Result<(), std::io::Error>>
where
    T: InstructionParser,
    T: std::marker::Send,
    T: 'static,
{
    std::thread::spawn(move || {
        debug!(
            "Inside the spawned thread: {:?}",
            std::thread::current().id()
        );
        machine.start()
    })
}

fn main() {
    env_logger::init();
    let rom_file = env::args().nth(1).expect("Please input a ROM file");
    let ins_parser = opcodes::OpcodeMaskParser {};
    let mut vm = core::Machine::new("Chip8", ins_parser);
    vm.load_rom(&rom_file)
        .expect("Unable to load ROM from file");
    debug!("{:#?}", vm);
    let handle = launch_thread(vm);
    if let Ok(_) = handle.join() {
        info!("Shutting down...");
    } else {
        error!("VM thread failed to launch!");
    }
}
