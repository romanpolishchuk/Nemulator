use crate::ppu::PPURegisters;

pub struct Memory {
    ram: Vec<u8>,
    pub ppu_registers: PPURegisters,
    apu_io: [u8; 32],
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    vram: Vec<u8>,
    palettes: Vec<u8>,
}

impl Memory {
    pub fn new(
        ram: Vec<u8>,
        ppu_registers: PPURegisters,
        apu_io: [u8; 32],
        prg_rom: Vec<u8>,
        chr_rom: Vec<u8>,
    ) -> Memory {
        Memory {
            ram,
            ppu_registers,
            apu_io,
            prg_rom,
            chr_rom,
            vram: vec![0; 2048],
            palettes: vec![0; 32],
        }
    }

    pub fn get(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize],
            0x2000..=0x3FFF => self.ppu_registers.get(address & 0x0007),
            0x4000..=0x401F => self.apu_io[(address - 0x4000) as usize],
            0x8000..=0xFFFF => self.prg_rom[(address - 0x8000) as usize % self.prg_rom.len()],
            _ => panic!("Invalid address: {:X}", address),
        }
    }

    pub fn ppu_get(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.chr_rom[address as usize],
            0x2000..=0x3EFF => self.vram[((address - 0x2000) % 2048) as usize],
            0x3F00..=0x3FFF => self.palettes[(address & 0b0001_1111) as usize],
            _ => 0,
        }
    }

    pub fn set(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize] = value,
            0x2000..=0x3FFF => {
                self.ppu_registers
                    .set(address, value, &mut self.vram, &mut self.palettes)
            }
            0x4000..=0x401F => self.apu_io[(address - 0x4000) as usize] = value,
            0x8000..=0xFFFF => {}
            _ => panic!("Invalid address: {:X}", address),
        };
    }
}
