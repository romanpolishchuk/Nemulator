mod cpu;
mod memory;
mod rom_reader;
use cpu::CPU;
use memory::Memory;
use raylib;
use raylib::prelude::*;

use crate::memory::PPURegisters;

struct Emulator {
    cpu: CPU,
    memory: Memory,
    cycle: u64,
}

impl Emulator {
    fn cycle(&mut self) {
        self.cpu.cycle(&mut self.memory, self.cycle).unwrap();
        self.cycle += 1;
    }
}

fn main() {
    let file = rom_reader::read_file("./assets/nestest.nes");
    let mut memory = Memory::new(
        vec![0; 0x800],
        PPURegisters::new(),
        [0; 32],
        file.prg_rom,
        file.chr_rom,
    );

    let mut emulator = Emulator {
        cpu: CPU::new(&mut memory, Some("log.txt")),
        memory,
        cycle: 7,
    };

    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_fps(0, 0);

        for table_index in 0..=1 as u16 {
            for y in 0..16 as u16 {
                for x in 0..16 as u16 {
                    for fine_y in 0..8 as u16 {
                        let address0 = (y << 8 | x << 4) | fine_y | (table_index << 12);
                        let address1 = (address0 | 1 << 3) | fine_y | (table_index << 12);

                        let row_half0 = emulator.memory.chr_rom[address0 as usize];
                        let row_half1 = emulator.memory.chr_rom[address1 as usize];

                        for fine_x in 0..8 as u16 {
                            let value = (row_half0 >> (7 - fine_x) & 0b0000_0001)
                                + ((row_half1 >> (7 - fine_x) & 0b0000_0001) << 1);

                            d.draw_rectangle(
                                (x * 8 + fine_x + 256 * table_index) as i32 * 2,
                                (y * 8 + fine_y) as i32 * 2,
                                2,
                                2,
                                Color::new(
                                    value * 85,
                                    value * 85,
                                    value * 85,
                                    if value == 0 { 0 } else { 255 },
                                ),
                            );
                        }
                    }
                }
            }
        }

        emulator.cycle();
    }
}
