mod cpu;
mod memory;
mod ppu;
mod rom_reader;
use cpu::CPU;
use memory::Memory;
use raylib;
use raylib::prelude::*;

use crate::ppu::{PPURegisters, ppu_cycle};

struct Emulator {
    cpu: CPU,
    memory: Memory,
    cpu_cycle: u64,
    ppu_cycle: u64,
}

impl Emulator {
    fn cycle(&mut self, d: &mut RaylibDrawHandle) {
        if self.ppu_cycle % 3 == 0 {
            self.cpu.cycle(&mut self.memory, self.cpu_cycle).unwrap();
            self.cpu_cycle += 1;
        }
        ppu_cycle(&mut self.memory, self.ppu_cycle, d);
        self.ppu_cycle += 1;
    }
}

fn main() {
    let file = rom_reader::read_file("./assets/tests/color_test.nes");
    let mut memory = Memory::new(
        vec![0; 0x800],
        PPURegisters::new(),
        [0; 32],
        file.prg_rom,
        file.chr_rom,
    );

    let mut emulator = Emulator {
        cpu: CPU::new(&mut memory, None),
        memory,
        cpu_cycle: 7,
        ppu_cycle: 0,
    };

    let (mut rl, thread) = raylib::init()
        .size(341 * 2, 262 * 2)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        for i in 0..(341 * 262) {
            emulator.cycle(&mut d);
        }
    }
}
