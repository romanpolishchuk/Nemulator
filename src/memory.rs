pub struct PPURegisters {
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    ppuscroll: u16,
    ppuaddr: u16,
    ppudata_buffer: u8,

    w: bool,
}

impl PPURegisters {
    pub fn new() -> PPURegisters {
        PPURegisters {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0b10100000,
            oamaddr: 0,
            w: false,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata_buffer: 0,
        }
    }

    fn get_ppuctrl_increment_mode(&self) -> bool {
        self.ppuctrl & 0b0000_0100 != 0
    }

    fn set_ppustatus_vblank(&mut self, value: bool) {
        if value {
            self.ppustatus |= 0b1000_0000;
        } else {
            self.ppustatus &= 0b0111_1111;
        }
    }

    fn set(&mut self, address: u16, value: u8, chr_rom: &mut Vec<u8>) {
        match address & 0x0007 {
            0 => self.ppuctrl = value,
            1 => self.ppumask = value,
            2 => {}
            6 => {
                if !self.w {
                    self.ppuaddr &= 0x0F;
                    self.ppuaddr |= (value as u16) << 8;
                    self.w = true;
                } else {
                    self.ppuaddr &= 0xF0;
                    self.ppuaddr |= value as u16;
                    self.w = false;
                }
            }
            7 => {
                chr_rom[self.ppuaddr as usize] = value;

                if self.get_ppuctrl_increment_mode() {
                    self.ppuaddr += 32;
                } else {
                    self.ppuaddr += 1;
                }
            }

            _ => todo!(),
        }
    }

    fn get(&mut self, address: u16) -> u8 {
        match address & 0x0007 {
            2 => {
                let ret_value = self.ppustatus;
                self.set_ppustatus_vblank(false);
                // TODO: Add open bus behaviour, see https://www.nesdev.org/wiki/PPU_registers#PPUSTATUS

                ret_value
            }

            _ => 0,
        }
    }
}

pub struct Memory {
    ram: Vec<u8>,
    ppu_registers: PPURegisters,
    apu_io: [u8; 32],
    prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
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

    pub fn set(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize] = value,
            0x2000..=0x3FFF => self.ppu_registers.set(address, value, &mut self.chr_rom),
            0x4000..=0x401F => self.apu_io[(address - 0x4000) as usize] = value,
            0x8000..=0xFFFF => {}
            _ => panic!("Invalid address: {:X}", address),
        };
    }
}
