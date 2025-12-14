use crate::memory::Memory;
use raylib::prelude::*;

pub struct PPURegisters {
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    ppuscroll: u16,
    ppuaddr: u16,
    ppudata_buffer: u8,

    w: bool,
    draw_nametable: u8,
    draw_attr_table: u8,
    draw_pattern_table_lo: u8,
    draw_pattern_table_hi: u8,

    nametable_latch: u8,
    attr_table_latch: u8,
    pattern_table_lo_latch: u8,
    pattern_table_hi_latch: u8,
}

impl PPURegisters {
    pub fn new() -> PPURegisters {
        PPURegisters {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0b10100000,
            oamaddr: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata_buffer: 0,

            w: false,
            draw_nametable: 0,
            draw_attr_table: 0,
            draw_pattern_table_lo: 0,
            draw_pattern_table_hi: 0,

            nametable_latch: 0,
            attr_table_latch: 0,
            pattern_table_lo_latch: 0,
            pattern_table_hi_latch: 0,
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

    pub fn set(&mut self, address: u16, value: u8, vram: &mut Vec<u8>, palettes: &mut Vec<u8>) {
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
                match self.ppuaddr {
                    0x2000..=0x2FFF => vram[((self.ppuaddr - 0x2000) % 2048) as usize] = value,
                    0x3F00..=0x3FFF => {
                        palettes[((self.ppuaddr - 0x3F00) & 0b0001_1111) as usize] = value
                    }
                    _ => {}
                }

                if self.get_ppuctrl_increment_mode() {
                    self.ppuaddr += 32;
                } else {
                    self.ppuaddr += 1;
                }
            }

            _ => todo!(),
        }
    }

    pub fn get(&mut self, address: u16) -> u8 {
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

const ppu_colors: [Color; 1] = [Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
}];

pub fn ppu_cycle(memory: &mut Memory, cycle: u64, d: &mut RaylibDrawHandle) {
    let scanline = cycle / 341;
    let pixel = cycle % 341;

    if pixel == 0 {
        return;
    }

    let tile_fine_x = pixel % 8;

    if tile_fine_x == 0 {
        memory.ppu_registers.draw_nametable = memory.ppu_registers.nametable_latch;
        memory.ppu_registers.draw_pattern_table_hi = memory.ppu_registers.pattern_table_hi_latch;
        memory.ppu_registers.draw_pattern_table_lo = memory.ppu_registers.pattern_table_lo_latch;
        memory.ppu_registers.draw_attr_table = memory.ppu_registers.attr_table_latch;
    } else if tile_fine_x == 1 {
        memory.ppu_registers.nametable_latch =
            memory.ppu_get(((scanline / 8) * 32 + pixel / 8) as u16);
    } else if tile_fine_x == 3 {
        memory.ppu_registers.attr_table_latch =
            memory.ppu_get((0x23C0 + (scanline / 32) * 8 + pixel / 32) as u16);
    } else if tile_fine_x == 5 {
        memory.ppu_registers.pattern_table_lo_latch =
            memory.ppu_get(memory.ppu_registers.nametable_latch as u16);
    } else if tile_fine_x == 7 {
        memory.ppu_registers.pattern_table_hi_latch =
            memory.ppu_get(memory.ppu_registers.nametable_latch as u16 + 8);
    }

    let pixel_data = ((memory.ppu_registers.draw_pattern_table_lo >> tile_fine_x) & 1)
        + ((memory.ppu_registers.draw_pattern_table_hi >> tile_fine_x) & 1) * 2;
    let palette_color = memory
        .ppu_get(0x3f00 + memory.ppu_registers.draw_attr_table as u16 * 4 + pixel_data as u16);
    let debug_color: u32 = (palette_color as u32).wrapping_mul(364353);

    d.draw_rectangle(
        (pixel * 2) as i32,
        (scanline * 2) as i32,
        2,
        2,
        Color {
            r: (debug_color % 256) as u8,
            g: ((debug_color / 256) % 256) as u8,
            b: ((debug_color / 256 / 256) % 256) as u8,
            a: 255,
        },
    );
}
