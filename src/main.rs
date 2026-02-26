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
        //ppu_cycle(&mut self.memory, self.ppu_cycle, d);
        self.ppu_cycle += 1;
    }

    fn draw_debug(&self, d: &mut RaylibDrawHandle) {
        // Draw pattern table
        for tile_index in 0..256 {
            for y in 0..8 {
                let index = tile_index * 16 + y;
                let pat_lo = self.memory.ppu_get(index);
                let pat_hi = self.memory.ppu_get(index + 8);
                for x in 0..8 {
                    let pixel_color = if (pat_lo & (0b1000_0000 >> x)) != 0 { 1 } else { 0 } + if (pat_hi & (0b1000_0000 >> x)) != 0 { 2 } else { 0 };
                    let color = Color { r: 85 * pixel_color, g: 85 * pixel_color, b: 85 * pixel_color, a: 255 };
                    //let color = Color { r: tile_index as u8, g: x * 16, b: y as u8 * 16, a: 255 };
                    d.draw_rectangle(((tile_index as i32 % 16) * 8 + x as i32 + 341) * 2, (y as i32 + (tile_index as i32 / 16) * 8) * 2, 2, 2, color);
                }
            }
        }
    }
}

fn main() {
    let file = rom_reader::read_file("./assets/tests/nestest.nes");
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
        .size((341 + 8*16) * 2, 262 * 2)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        for _ in 0..(341 * 262) {
            emulator.cycle(&mut d);
        }
        emulator.draw_debug(&mut d);
    }
}
