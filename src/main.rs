mod cpu;
mod rom_reader;
use cpu::CPU;

struct Memory {
    ram: Vec<u8>,
    ppu_registers: [u8; 8],
    apu_io: [u8; 32],
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl Memory {
    fn get(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize],
            0x2000..=0x3FFF => self.ppu_registers[(address & 0x0007) as usize],
            0x4000..=0x401F => self.apu_io[(address - 0x4000) as usize],
            0x8000..=0xFFFF => self.prg_rom[(address - 0x8000) as usize % self.prg_rom.len()],
            _ => todo!(),
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize] = value,
            0x2000..=0x3FFF => self.ppu_registers[(address & 0x0007) as usize] = value,
            0x4000..=0x401F => self.apu_io[(address - 0x4000) as usize] = value,
            0x8000..=0xFFFF => {}
            _ => todo!(),
        };
    }
}

struct Emulator {
    cpu: CPU,
    memory: Memory,
    cycle: u64,
}

impl Emulator {
    fn cycle(&mut self) {
        self.cpu.cycle(&mut self.memory, self.cycle);
        self.cycle += 1;
    }
}

fn main() {
    let file = rom_reader::read_file("./assets/nestest.nes");
    let mut memory = Memory {
        ram: vec![0; 0x800],
        ppu_registers: [0; 8],
        apu_io: [0; 32],
        prg_rom: file.prg_rom,
        chr_rom: file.chr_rom,
    };

    let mut emulator = Emulator {
        cpu: CPU::new(&memory),
        memory,
        cycle: 7,
    };

    emulator.cycle();
}
