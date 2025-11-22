mod opcodes;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::result;

use opcodes::OP;

use crate::Memory;

enum OPMode {
    A,
    Abs,
    AbsX,
    AbsY,
    Imm,
    Impl,
    Ind,
    XInd,
    IndY,
    Rel,
    Zpg,
    ZpgX,
    ZpgY,
}

pub struct CPU {
    accumulator: u8,
    index_x: u8,
    index_y: u8,
    program_counter: u16,
    stack_pointer: u8,
    status_register: u8,
    cycle: u64,
    log_file: File,
}

impl CPU {
    pub fn new(memory: &Memory, log_name: &str) -> CPU {
        fs::create_dir_all("./logs/").unwrap();
        let log_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(Path::new("./logs/").join(log_name))
            .unwrap();

        CPU {
            accumulator: 0,
            index_x: 0,
            index_y: 0,
            program_counter: u16::from_le_bytes([memory.get(0xFFFC), memory.get(0xFFFD)]),
            stack_pointer: 0xFD,
            status_register: 0b0010_0000,
            cycle: 7,
            log_file,
        }
    }

    fn get_flag_carry(&self) -> bool {
        self.status_register & 0b1 == 0b1
    }
    fn set_flag_carry(&mut self, value: bool) {
        if value {
            self.status_register |= 0b1;
        } else {
            self.status_register &= 0b1111_1110;
        }
    }

    fn get_flag_zero(&self) -> bool {
        self.status_register & 0b10 == 0b10
    }
    fn set_flag_zero(&mut self, value: bool) {
        if value {
            self.status_register |= 0b10;
        } else {
            self.status_register &= 0b1111_1101;
        }
    }

    fn get_flag_interrupt_disable(&self) -> bool {
        self.status_register & 0b100 == 0b100
    }
    fn set_flag_interrupt_disable(&mut self, value: bool) {
        if value {
            self.status_register |= 0b100;
        } else {
            self.status_register &= 0b1111_1011;
        }
    }

    fn get_flag_decimal(&self) -> bool {
        self.status_register & 0b1000 == 0b1000
    }
    fn set_flag_decimal(&mut self, value: bool) {
        if value {
            self.status_register |= 0b1000;
        } else {
            self.status_register &= 0b1111_0111;
        }
    }

    fn get_flag_b(&self) -> bool {
        self.status_register & 0b1000_0 == 0b1000_0
    }
    fn set_flag_b(&mut self, value: bool) {
        if value {
            self.status_register |= 0b1000_0;
        } else {
            self.status_register &= 0b1110_1111;
        }
    }

    fn get_flag_overflow(&self) -> bool {
        self.status_register & 0b1000_00 == 0b1000_000
    }
    fn set_flag_overflow(&mut self, value: bool) {
        if value {
            self.status_register |= 0b1000_000;
        } else {
            self.status_register &= 0b1011_1111;
        }
    }

    fn get_flag_negative(&self) -> bool {
        self.status_register & 0b1000_0000 == 0b1000_0000
    }
    fn set_flag_negative(&mut self, value: bool) {
        if value {
            self.status_register |= 0b1000_0000;
        } else {
            self.status_register &= 0b0111_1111;
        }
    }

    fn log_instr(&mut self, memory: &Memory, mode: OPMode) {
        let bytes = vec![
            memory.get(self.program_counter),
            memory.get(self.program_counter + 1),
            memory.get(self.program_counter + 2),
        ];

        let mut line = String::from("");
        line += &format!("{:04X}  ", self.program_counter);
        let name = OP::from(bytes[0]).to_string();
        line += &format!(
            "{:42}",
            match mode {
                OPMode::A => format!("{:02X}       {} A", bytes[0], name),
                OPMode::Abs => format!(
                    "{:02X} {:02X} {:02X} {} ${:02X}{:02X}",
                    bytes[0], bytes[1], bytes[2], name, bytes[2], bytes[1]
                ),
                OPMode::AbsX => format!(
                    "{:02X} {:02X} {:02X} {} ${:02X}{:02X}, X",
                    bytes[0], bytes[1], bytes[2], name, bytes[2], bytes[1]
                ),
                OPMode::AbsY => format!(
                    "{:02X} {:02X} {:02X} {} ${:02X}{:02X}, Y",
                    bytes[0], bytes[1], bytes[2], name, bytes[2], bytes[1]
                ),
                OPMode::Imm => format!(
                    "{:02X} {:02X}    {} #${:02X}",
                    bytes[0], bytes[1], name, bytes[1]
                ),
                OPMode::Impl => format!("{:02X}       {}", bytes[0], name),
                OPMode::Ind => format!(
                    "{:02X} {:02X} {:02X} {} (${:02X}{:02X})",
                    bytes[0], bytes[1], bytes[2], name, bytes[2], bytes[1]
                ),
                OPMode::XInd => format!(
                    "{:02X} {:02X}    {} (${:02X},X)",
                    bytes[0], bytes[1], name, bytes[1]
                ),
                OPMode::IndY => format!(
                    "{:02X} {:02X}    {} (${:02X}),Y",
                    bytes[0], bytes[1], name, bytes[1]
                ),
                OPMode::Rel => format!(
                    "{:02X} {:02X}    {} $({:04X})",
                    bytes[0],
                    bytes[1],
                    name,
                    (self.program_counter as i32 + 2 as i32 + (bytes[1] as i8) as i32) as u16
                ),
                OPMode::Zpg => format!(
                    "{:02X} {:02X}    {} ${:02X}",
                    bytes[0], bytes[1], name, bytes[1]
                ),
                OPMode::ZpgX => format!(
                    "{:02X} {:02X}    {} ${:02X},X",
                    bytes[0], bytes[1], name, bytes[1]
                ),
                OPMode::ZpgY => format!(
                    "{:02X} {:02X}    {} ${:02X},Y",
                    bytes[0], bytes[1], name, bytes[1]
                ),
            }
        );

        line += &format!(
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:  0,  0 CYC:{}",
            self.accumulator,
            self.index_x,
            self.index_y,
            self.status_register,
            self.stack_pointer,
            self.cycle
        );
        writeln!(self.log_file, "{}", line).unwrap();
    }

    fn abs_rmw<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::Abs);
            self.cycle += 6;

            return None;
        }

        let address_lb = memory.get(self.program_counter);
        self.program_counter += 1;
        let address_hb = memory.get(self.program_counter);
        self.program_counter += 1;

        let address = u16::from_le_bytes([address_lb, address_hb]);

        let value = memory.get(address);
        let result = callback(value);

        memory.set(address, result);

        Some((value, result))
    }

    fn absx_rmw<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::AbsX);
            self.cycle += 7;

            return None;
        }

        let address_lb = memory.get(self.program_counter);
        self.program_counter += 1;
        let address_hb = memory.get(self.program_counter);
        self.program_counter += 1;

        let mut address = u16::from_le_bytes([address_lb, address_hb]);
        address += self.index_x as u16;

        let value = memory.get(address);
        let result = callback(value);

        memory.set(address, result);

        Some((value, result))
    }

    fn zpg_rmw<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::Zpg);
            self.cycle += 5;
            return None;
        }

        let address = memory.get(self.program_counter);
        self.program_counter += 1;

        let value = memory.get(address.into());
        let result = callback(value);

        memory.set(address.into(), result);

        Some((value, result))
    }

    fn zpgx_rmw<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::ZpgX);
            self.cycle += 6;
            return None;
        }

        let mut address = memory.get(self.program_counter);
        self.program_counter += 1;

        address += self.index_x;

        let value = memory.get(address.into());
        let result = callback(value);

        memory.set(address.into(), result);

        Some((value, result))
    }

    fn acc<F>(&mut self, memory: &mut Memory, emulator_cycle: u64, callback: F) -> Option<(u8, u8)>
    where
        F: Fn(u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::A);
            self.cycle += 2;

            return None;
        }

        let value = self.accumulator;
        let result = callback(value);

        self.accumulator = result;

        Some((value, result))
    }

    fn abs_r<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        register: u8,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8, u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::Abs);
            self.cycle += 4;
            return None;
        }

        let address_lb = memory.get(self.program_counter);
        self.program_counter += 1;
        let address_hb = memory.get(self.program_counter);
        self.program_counter += 1;

        let address = u16::from_le_bytes([address_lb, address_hb]);

        let value = memory.get(address);
        let result = callback(register, memory.get(address));

        Some((value, result))
    }

    fn absx_r<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        register: u8,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8, u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::AbsX);

            let (_, is_overflow) = memory
                .get(self.program_counter)
                .overflowing_add(self.index_x);
            if is_overflow {
                self.cycle += 5;
            } else {
                self.cycle += 4;
            }

            return None;
        }

        let address_lb = memory.get(self.program_counter);
        self.program_counter += 1;
        let address_hb = memory.get(self.program_counter);
        self.program_counter += 1;

        let mut address = u16::from_le_bytes([address_lb, address_hb]);
        address += self.index_x as u16;

        let value = memory.get(address);
        let result = callback(register, memory.get(address));

        Some((value, result))
    }

    fn absy_r<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        register: u8,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8, u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::AbsY);

            let (_, is_overflow) = memory
                .get(self.program_counter)
                .overflowing_add(self.index_y);
            if is_overflow {
                self.cycle += 5;
            } else {
                self.cycle += 4;
            }

            return None;
        }

        let address_lb = memory.get(self.program_counter);
        self.program_counter += 1;
        let address_hb = memory.get(self.program_counter);
        self.program_counter += 1;

        let mut address = u16::from_le_bytes([address_lb, address_hb]);
        address += self.index_y as u16;

        let value = memory.get(address);
        let result = callback(register, memory.get(address));

        Some((value, result))
    }

    fn xind_r<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        register: u8,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8, u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::Ind);
            self.cycle += 6;
            return None;
        }

        let mut lookup = memory.get(self.program_counter);
        self.program_counter += 1;

        lookup += self.index_x;

        let address =
            u16::from_le_bytes([memory.get(lookup as u16), memory.get(lookup as u16 + 1)]);

        let value = memory.get(address);
        let result = callback(register, memory.get(address));

        Some((value, result))
    }

    fn imm_r<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        register: u8,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8, u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::Imm);
            self.cycle += 2;
            return None;
        }

        let imm = memory.get(self.program_counter);
        self.program_counter += 1;

        let value = imm;
        let result = callback(register, imm);

        Some((value, result))
    }

    fn indy_r<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        register: u8,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8, u8) -> u8,
    {
        let lookup = memory.get(self.program_counter);
        let mut lo = memory.get(lookup as u16);
        let mut hi = memory.get(lookup as u16 + 1);
        let overflow: bool;
        (lo, overflow) = lo.overflowing_add(self.index_y);
        if overflow {
            hi += 1;
        }

        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::IndY);
            self.cycle += 5;
            if overflow {
                self.cycle += 1;
            }

            return None;
        }

        self.program_counter += 1;

        let address = u16::from_le_bytes([lo, hi]);

        let value = memory.get(address);
        let result = callback(register, memory.get(address));

        Some((value, result))
    }

    fn zpg_r<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        register: u8,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8, u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::Zpg);
            self.cycle += 3;

            return None;
        }

        let lookup = memory.get(self.program_counter);
        let address = lookup as u16;
        self.program_counter += 1;

        let value = memory.get(address);
        let result = callback(register, memory.get(address));

        Some((value, result))
    }

    fn zpgx_r<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        register: u8,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8, u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::ZpgX);
            self.cycle += 4;

            return None;
        }

        let lookup = memory.get(self.program_counter).wrapping_add(self.index_x);
        self.program_counter += 1;
        let address = lookup as u16;

        let value = memory.get(address);
        let result = callback(register, memory.get(address));

        Some((value, result))
    }

    fn zpgy_r<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        register: u8,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8, u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::ZpgY);
            self.cycle += 4;

            return None;
        }

        let lookup = memory.get(self.program_counter).wrapping_add(self.index_y);
        self.program_counter += 1;
        let address = lookup as u16;

        let value = memory.get(address);
        let result = callback(register, memory.get(address));

        Some((value, result))
    }

    fn xind_w(&mut self, memory: &mut Memory, emulator_cycle: u64, register: u8) {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::XInd);
            self.cycle += 6;
            return;
        }

        let lookup = memory.get(self.program_counter) + self.index_x;
        self.program_counter += 1;

        let address =
            u16::from_le_bytes([memory.get(lookup as u16), memory.get(lookup as u16 + 1)]);

        memory.set(address, register);
    }

    fn abs_w(&mut self, memory: &mut Memory, emulator_cycle: u64, register: u8) {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::Abs);
            self.cycle += 4;
            return;
        }

        let lo = memory.get(self.program_counter);
        self.program_counter += 1;
        let hi = memory.get(self.program_counter);
        self.program_counter += 1;
        let address = u16::from_le_bytes([lo, hi]);
        memory.set(address, register);
    }

    fn absx_w(&mut self, memory: &mut Memory, emulator_cycle: u64, register: u8) {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::AbsX);
            self.cycle += 5;
            return;
        }

        let lo = memory.get(self.program_counter);
        self.program_counter += 1;
        let hi = memory.get(self.program_counter);
        self.program_counter += 1;
        let address = u16::from_le_bytes([lo, hi]) + self.index_x as u16;
        memory.set(address, register);
    }

    fn absy_w(&mut self, memory: &mut Memory, emulator_cycle: u64, register: u8) {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::AbsY);
            self.cycle += 5;
            return;
        }

        let lo = memory.get(self.program_counter);
        self.program_counter += 1;
        let hi = memory.get(self.program_counter);
        self.program_counter += 1;
        let address = u16::from_le_bytes([lo, hi]) + self.index_y as u16;
        memory.set(address, register);
    }

    fn indy_w(&mut self, memory: &mut Memory, emulator_cycle: u64, register: u8) {
        let lookup = memory.get(self.program_counter);
        let mut lo = memory.get(lookup as u16);
        let mut hi = memory.get(lookup as u16 + 1);
        let overflow: bool;
        (lo, overflow) = lo.overflowing_add(self.index_y);
        if overflow {
            hi += 1;
        }

        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::IndY);
            self.cycle += 5;
            if overflow {
                self.cycle += 1;
            }

            return;
        }

        self.program_counter += 1;

        let address = u16::from_le_bytes([lo, hi]);

        memory.set(address, register);
    }

    fn zpg_w(&mut self, memory: &mut Memory, emulator_cycle: u64, register: u8) {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::Zpg);
            self.cycle += 3;
            return;
        }

        let address = memory.get(self.program_counter) as u16;
        self.program_counter += 1;

        memory.set(address, register);
    }

    fn zpgx_w(&mut self, memory: &mut Memory, emulator_cycle: u64, register: u8) {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::ZpgX);
            self.cycle += 3;
            return;
        }

        let address = (memory.get(self.program_counter).wrapping_add(self.index_x)) as u16;
        self.program_counter += 1;

        memory.set(address, register);
    }

    fn zpgy_w(&mut self, memory: &mut Memory, emulator_cycle: u64, register: u8) {
        if self.cycle == emulator_cycle {
            self.program_counter -= 1;
            self.log_instr(memory, OPMode::ZpgY);
            self.cycle += 3;
            return;
        }

        let address = (memory.get(self.program_counter).wrapping_add(self.index_y)) as u16;
        self.program_counter += 1;

        memory.set(address, register);
    }

    pub fn cycle(&mut self, memory: &mut Memory, emulator_cycle: u64) {
        if self.cycle - 1 > emulator_cycle {
            return;
        }

        let op = memory.get(self.program_counter);
        self.program_counter += 1;

        match OP::from(op) {
            OP::ADC_X_ind
            | OP::ADC_abs
            | OP::ADC_abs_X
            | OP::ADC_abs_Y
            | OP::ADC_imm
            | OP::ADC_ind_Y
            | OP::ADC_zpg
            | OP::ADC_zpg_X => {
                let offset: u8 = if self.get_flag_carry() { 1 } else { 0 };
                let callback = |acc, x| acc + x + offset;
                let register = self.accumulator;
                if let Some((value, result)) = match OP::from(op) {
                    OP::ADC_X_ind => self.xind_r(memory, emulator_cycle, register, callback),
                    OP::ADC_abs => self.abs_r(memory, emulator_cycle, register, callback),
                    OP::ADC_abs_X => self.absx_r(memory, emulator_cycle, register, callback),
                    OP::ADC_abs_Y => self.absy_r(memory, emulator_cycle, register, callback),
                    OP::ADC_imm => self.imm_r(memory, emulator_cycle, register, callback),
                    OP::ADC_ind_Y => self.indy_r(memory, emulator_cycle, register, callback),
                    OP::ADC_zpg => self.zpg_r(memory, emulator_cycle, register, callback),
                    OP::ADC_zpg_X | _ => self.zpgx_r(memory, emulator_cycle, register, callback),
                } {
                    self.set_flag_carry(result < register);
                    self.set_flag_zero(result == 0);
                    self.set_flag_overflow(
                        (result ^ register) & (result ^ value) & 0b1000_0000 != 0,
                    );
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                    self.accumulator = result;
                }
            }

            OP::ALR_imm => todo!("{:#04X}", op),

            OP::ANC_imm_0x0b => todo!("{:#04X}", op),
            OP::ANC_imm_0x2b => todo!("{:#04X}", op),

            OP::AND_X_ind
            | OP::AND_abs
            | OP::AND_abs_X
            | OP::AND_abs_Y
            | OP::AND_imm
            | OP::AND_ind_Y
            | OP::AND_zpg
            | OP::AND_zpg_X => {
                let callback = |reg, x| reg & x;
                let register = self.accumulator;
                if let Some((_, result)) = match OP::from(op) {
                    OP::AND_X_ind => self.xind_r(memory, emulator_cycle, register, callback),
                    OP::AND_abs => self.abs_r(memory, emulator_cycle, register, callback),
                    OP::AND_abs_X => self.absx_r(memory, emulator_cycle, register, callback),
                    OP::AND_abs_Y => self.absy_r(memory, emulator_cycle, register, callback),
                    OP::AND_imm => self.imm_r(memory, emulator_cycle, register, callback),
                    OP::AND_ind_Y => self.indy_r(memory, emulator_cycle, register, callback),
                    OP::AND_zpg => self.zpg_r(memory, emulator_cycle, register, callback),
                    OP::AND_zpg_X | _ => self.zpgx_r(memory, emulator_cycle, register, callback),
                } {
                    self.accumulator = result;
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                }
            }

            OP::ANE_imm => todo!("{:#04X}", op),

            OP::ARR_imm => todo!("{:#04X}", op),

            OP::ASL_A | OP::ASL_abs | OP::ASL_abs_X | OP::ASL_zpg | OP::ASL_zpg_X => {
                let callback = |x| x << 1;
                if let Some((value, result)) = match OP::from(op) {
                    OP::ASL_A => self.acc(memory, emulator_cycle, callback),
                    OP::ASL_abs => self.abs_rmw(memory, emulator_cycle, callback),
                    OP::ASL_abs_X => self.absx_rmw(memory, emulator_cycle, callback),
                    OP::ASL_zpg => self.zpg_rmw(memory, emulator_cycle, callback),
                    OP::ASL_zpg_X | _ => self.zpgx_rmw(memory, emulator_cycle, callback),
                } {
                    self.set_flag_carry(value & 0b1000_0000 != 0);
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                }
            }

            OP::BCC_rel => todo!("{:#04X}", op),

            OP::BCS_rel => todo!("{:#04X}", op),

            OP::BEQ_rel => todo!("{:#04X}", op),

            OP::BIT_abs => todo!("{:#04X}", op),
            OP::BIT_zpg => todo!("{:#04X}", op),

            OP::BMI_rel => todo!("{:#04X}", op),

            OP::BNE_rel => todo!("{:#04X}", op),

            OP::BPL_rel => todo!("{:#04X}", op),

            OP::BRK_impl => todo!("{:#04X}", op),

            OP::BVC_rel => todo!("{:#04X}", op),

            OP::BVS_rel => todo!("{:#04X}", op),

            OP::CLC_impl => todo!("{:#04X}", op),

            OP::CLD_impl => todo!("{:#04X}", op),

            OP::CLI_impl => todo!("{:#04X}", op),

            OP::CLV_impl => todo!("{:#04X}", op),

            OP::CMP_X_ind => todo!("{:#04X}", op),
            OP::CMP_abs => todo!("{:#04X}", op),
            OP::CMP_abs_X => todo!("{:#04X}", op),
            OP::CMP_abs_Y => todo!("{:#04X}", op),
            OP::CMP_imm => todo!("{:#04X}", op),
            OP::CMP_ind_Y => todo!("{:#04X}", op),
            OP::CMP_zpg => todo!("{:#04X}", op),
            OP::CMP_zpg_X => todo!("{:#04X}", op),

            OP::CPX_abs => todo!("{:#04X}", op),
            OP::CPX_imm => todo!("{:#04X}", op),
            OP::CPX_zpg => todo!("{:#04X}", op),

            OP::CPY_abs => todo!("{:#04X}", op),
            OP::CPY_imm => todo!("{:#04X}", op),
            OP::CPY_zpg => todo!("{:#04X}", op),

            OP::DCP_X_ind => todo!("{:#04X}", op),
            OP::DCP_abs => todo!("{:#04X}", op),
            OP::DCP_abs_X => todo!("{:#04X}", op),
            OP::DCP_abs_Y => todo!("{:#04X}", op),
            OP::DCP_ind_Y => todo!("{:#04X}", op),
            OP::DCP_zpg => todo!("{:#04X}", op),
            OP::DCP_zpg_X => todo!("{:#04X}", op),

            OP::DEC_abs | OP::DEC_abs_X | OP::DEC_zpg | OP::DEC_zpg_X => {
                let callback = |x| x - 1;
                if let Some((_, result)) = match OP::from(op) {
                    OP::DEC_abs => self.abs_rmw(memory, emulator_cycle, callback),
                    OP::DEC_abs_X => self.absx_rmw(memory, emulator_cycle, callback),
                    OP::DEC_zpg => self.zpg_rmw(memory, emulator_cycle, callback),
                    OP::DEC_zpg_X | _ => self.zpgx_rmw(memory, emulator_cycle, callback),
                } {
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                }
            }

            OP::DEX_impl => todo!("{:#04X}", op),

            OP::DEY_impl => todo!("{:#04X}", op),

            OP::EOR_X_ind
            | OP::EOR_abs
            | OP::EOR_abs_X
            | OP::EOR_abs_Y
            | OP::EOR_imm
            | OP::EOR_ind_Y
            | OP::EOR_zpg
            | OP::EOR_zpg_X => {
                let callback = |reg, x| reg ^ x;
                let register = self.accumulator;
                if let Some((_, result)) = match OP::from(op) {
                    OP::EOR_X_ind => self.xind_r(memory, emulator_cycle, register, callback),
                    OP::EOR_abs => self.abs_r(memory, emulator_cycle, register, callback),
                    OP::EOR_abs_X => self.absx_r(memory, emulator_cycle, register, callback),
                    OP::EOR_abs_Y => self.absy_r(memory, emulator_cycle, register, callback),
                    OP::EOR_imm => self.imm_r(memory, emulator_cycle, register, callback),
                    OP::EOR_ind_Y => self.indy_r(memory, emulator_cycle, register, callback),
                    OP::EOR_zpg => self.zpg_r(memory, emulator_cycle, register, callback),
                    OP::EOR_zpg_X | _ => self.zpgx_r(memory, emulator_cycle, register, callback),
                } {
                    self.accumulator = result;
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                }
            }

            OP::INC_abs | OP::INC_abs_X | OP::INC_zpg | OP::INC_zpg_X => {
                let callback = |x| x + 1;
                if let Some((_, result)) = match OP::from(op) {
                    OP::INC_abs => self.abs_rmw(memory, emulator_cycle, callback),
                    OP::INC_abs_X => self.absx_rmw(memory, emulator_cycle, callback),
                    OP::INC_zpg => self.zpg_rmw(memory, emulator_cycle, callback),
                    OP::INC_zpg_X | _ => self.zpgx_rmw(memory, emulator_cycle, callback),
                } {
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                }
            }

            OP::INX_impl => todo!("{:#04X}", op),

            OP::INY_impl => todo!("{:#04X}", op),

            OP::ISC_X_ind => todo!("{:#04X}", op),
            OP::ISC_abs => todo!("{:#04X}", op),
            OP::ISC_abs_X => todo!("{:#04X}", op),
            OP::ISC_abs_Y => todo!("{:#04X}", op),
            OP::ISC_ind_Y => todo!("{:#04X}", op),
            OP::ISC_zpg => todo!("{:#04X}", op),
            OP::ISC_zpg_X => todo!("{:#04X}", op),

            OP::JAM_0x12 => todo!("{:#04X}", op),
            OP::JAM_0x2 => todo!("{:#04X}", op),
            OP::JAM_0x22 => todo!("{:#04X}", op),
            OP::JAM_0x32 => todo!("{:#04X}", op),
            OP::JAM_0x42 => todo!("{:#04X}", op),
            OP::JAM_0x52 => todo!("{:#04X}", op),
            OP::JAM_0x62 => todo!("{:#04X}", op),
            OP::JAM_0x72 => todo!("{:#04X}", op),
            OP::JAM_0x92 => todo!("{:#04X}", op),
            OP::JAM_0xb2 => todo!("{:#04X}", op),
            OP::JAM_0xd2 => todo!("{:#04X}", op),
            OP::JAM_0xf2 => todo!("{:#04X}", op),

            OP::JMP_abs => {
                if self.cycle == emulator_cycle {
                    self.cycle += 3;
                    self.program_counter -= 1;
                    return;
                }

                self.program_counter = u16::from_le_bytes([
                    memory.get(self.program_counter),
                    memory.get(self.program_counter + 1),
                ]);
            }
            OP::JMP_ind => todo!("{:#04X}", op),

            OP::JSR_abs => todo!("{:#04X}", op),

            OP::LAS_abs_Y => todo!("{:#04X}", op),

            OP::LAX_X_ind => todo!("{:#04X}", op),
            OP::LAX_abs => todo!("{:#04X}", op),
            OP::LAX_abs_Y => todo!("{:#04X}", op),
            OP::LAX_ind_Y => todo!("{:#04X}", op),
            OP::LAX_zpg => todo!("{:#04X}", op),
            OP::LAX_zpg_Y => todo!("{:#04X}", op),

            OP::LDA_X_ind
            | OP::LDA_abs
            | OP::LDA_abs_X
            | OP::LDA_abs_Y
            | OP::LDA_imm
            | OP::LDA_ind_Y
            | OP::LDA_zpg
            | OP::LDA_zpg_X => {
                let callback = |_, x| x;
                let register = self.accumulator;
                if let Some((_, result)) = match OP::from(op) {
                    OP::LDA_X_ind => self.xind_r(memory, emulator_cycle, register, callback),
                    OP::LDA_abs => self.abs_r(memory, emulator_cycle, register, callback),
                    OP::LDA_abs_X => self.absx_r(memory, emulator_cycle, register, callback),
                    OP::LDA_abs_Y => self.absy_r(memory, emulator_cycle, register, callback),
                    OP::LDA_imm => self.imm_r(memory, emulator_cycle, register, callback),
                    OP::LDA_ind_Y => self.indy_r(memory, emulator_cycle, register, callback),
                    OP::LDA_zpg => self.zpg_r(memory, emulator_cycle, register, callback),
                    OP::LDA_zpg_X | _ => self.zpgx_r(memory, emulator_cycle, register, callback),
                } {
                    self.accumulator = result;
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                }
            }

            OP::LDX_abs | OP::LDX_abs_Y | OP::LDX_imm | OP::LDX_zpg | OP::LDX_zpg_Y => {
                let callback = |_, x| x;
                let register = self.index_x;
                if let Some((_, result)) = match OP::from(op) {
                    OP::LDX_abs => self.abs_r(memory, emulator_cycle, register, callback),
                    OP::LDX_abs_Y => self.absy_r(memory, emulator_cycle, register, callback),
                    OP::LDX_imm => self.imm_r(memory, emulator_cycle, register, callback),
                    OP::LDX_zpg => self.zpg_r(memory, emulator_cycle, register, callback),
                    OP::LDX_zpg_Y | _ => self.zpgy_r(memory, emulator_cycle, register, callback),
                } {
                    self.index_x = result;
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                }
            }

            OP::LDY_abs | OP::LDY_abs_X | OP::LDY_imm | OP::LDY_zpg | OP::LDY_zpg_X => {
                let callback = |_, x| x;
                let register = self.index_y;
                if let Some((_, result)) = match OP::from(op) {
                    OP::LDY_abs => self.abs_r(memory, emulator_cycle, register, callback),
                    OP::LDY_abs_X => self.absx_r(memory, emulator_cycle, register, callback),
                    OP::LDY_imm => self.imm_r(memory, emulator_cycle, register, callback),
                    OP::LDY_zpg => self.zpg_r(memory, emulator_cycle, register, callback),
                    OP::LDY_zpg_X | _ => self.zpgx_r(memory, emulator_cycle, register, callback),
                } {
                    self.index_y = result;
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                }
            }

            OP::LSR_A | OP::LSR_abs | OP::LSR_abs_X | OP::LSR_zpg | OP::LSR_zpg_X => {
                let callback = |x| x >> 1;
                if let Some((value, result)) = match OP::from(op) {
                    OP::LSR_A => self.acc(memory, emulator_cycle, callback),
                    OP::LSR_abs => self.abs_rmw(memory, emulator_cycle, callback),
                    OP::LSR_abs_X => self.absx_rmw(memory, emulator_cycle, callback),
                    OP::LSR_zpg => self.zpg_rmw(memory, emulator_cycle, callback),
                    OP::LSR_zpg_X | _ => self.zpgx_rmw(memory, emulator_cycle, callback),
                } {
                    self.set_flag_carry(value & 0b0000_0001 != 0);
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(false);
                }
            }

            OP::LXA_imm => todo!("{:#04X}", op),

            OP::NOP_abs_0xc => todo!("{:#04X}", op),
            OP::NOP_abs_X_0x1c => todo!("{:#04X}", op),
            OP::NOP_abs_X_0x3c => todo!("{:#04X}", op),
            OP::NOP_abs_X_0x5c => todo!("{:#04X}", op),
            OP::NOP_abs_X_0x7c => todo!("{:#04X}", op),
            OP::NOP_abs_X_0xdc => todo!("{:#04X}", op),
            OP::NOP_abs_X_0xfc => todo!("{:#04X}", op),
            OP::NOP_imm_0x80 => todo!("{:#04X}", op),
            OP::NOP_imm_0x82 => todo!("{:#04X}", op),
            OP::NOP_imm_0x89 => todo!("{:#04X}", op),
            OP::NOP_imm_0xc2 => todo!("{:#04X}", op),
            OP::NOP_imm_0xe2 => todo!("{:#04X}", op),
            OP::NOP_impl_0x1a => todo!("{:#04X}", op),
            OP::NOP_impl_0x3a => todo!("{:#04X}", op),
            OP::NOP_impl_0x5a => todo!("{:#04X}", op),
            OP::NOP_impl_0x7a => todo!("{:#04X}", op),
            OP::NOP_impl_0xda => todo!("{:#04X}", op),
            OP::NOP_impl_0xea => todo!("{:#04X}", op),
            OP::NOP_impl_0xfa => todo!("{:#04X}", op),
            OP::NOP_zpg_0x4 => todo!("{:#04X}", op),
            OP::NOP_zpg_0x44 => todo!("{:#04X}", op),
            OP::NOP_zpg_0x64 => todo!("{:#04X}", op),
            OP::NOP_zpg_X_0x14 => todo!("{:#04X}", op),
            OP::NOP_zpg_X_0x34 => todo!("{:#04X}", op),
            OP::NOP_zpg_X_0x54 => todo!("{:#04X}", op),
            OP::NOP_zpg_X_0x74 => todo!("{:#04X}", op),
            OP::NOP_zpg_X_0xd4 => todo!("{:#04X}", op),
            OP::NOP_zpg_X_0xf4 => todo!("{:#04X}", op),

            OP::ORA_X_ind
            | OP::ORA_abs
            | OP::ORA_abs_X
            | OP::ORA_abs_Y
            | OP::ORA_imm
            | OP::ORA_ind_Y
            | OP::ORA_zpg
            | OP::ORA_zpg_X => {
                let callback = |reg, x| reg | x;
                let register = self.accumulator;
                if let Some((_, result)) = match OP::from(op) {
                    OP::ORA_X_ind => self.xind_r(memory, emulator_cycle, register, callback),
                    OP::ORA_abs => self.abs_r(memory, emulator_cycle, register, callback),
                    OP::ORA_abs_X => self.absx_r(memory, emulator_cycle, register, callback),
                    OP::ORA_abs_Y => self.absy_r(memory, emulator_cycle, register, callback),
                    OP::ORA_imm => self.imm_r(memory, emulator_cycle, register, callback),
                    OP::ORA_ind_Y => self.indy_r(memory, emulator_cycle, register, callback),
                    OP::ORA_zpg => self.zpg_r(memory, emulator_cycle, register, callback),
                    OP::ORA_zpg_X | _ => self.zpgx_r(memory, emulator_cycle, register, callback),
                } {
                    self.accumulator = result;
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                }
            }

            OP::PHA_impl => todo!("{:#04X}", op),

            OP::PHP_impl => todo!("{:#04X}", op),

            OP::PLA_impl => todo!("{:#04X}", op),

            OP::PLP_impl => todo!("{:#04X}", op),

            OP::RLA_X_ind => todo!("{:#04X}", op),
            OP::RLA_abs => todo!("{:#04X}", op),
            OP::RLA_abs_X => todo!("{:#04X}", op),
            OP::RLA_abs_Y => todo!("{:#04X}", op),
            OP::RLA_ind_Y => todo!("{:#04X}", op),
            OP::RLA_zpg => todo!("{:#04X}", op),
            OP::RLA_zpg_X => todo!("{:#04X}", op),

            OP::ROL_A | OP::ROL_abs | OP::ROL_abs_X | OP::ROL_zpg | OP::ROL_zpg_X => {
                let carry = self.get_flag_carry();
                let callback = |x| (x << 1) | carry as u8;
                if let Some((value, result)) = match OP::from(op) {
                    OP::ROL_A => self.acc(memory, emulator_cycle, callback),
                    OP::ROL_abs => self.abs_rmw(memory, emulator_cycle, callback),
                    OP::ROL_abs_X => self.absx_rmw(memory, emulator_cycle, callback),
                    OP::ROL_zpg => self.zpg_rmw(memory, emulator_cycle, callback),
                    OP::ROL_zpg_X | _ => self.zpgx_rmw(memory, emulator_cycle, callback),
                } {
                    self.set_flag_carry(value & 0b1000_0000 != 0);
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                }
            }

            OP::ROR_A | OP::ROR_abs | OP::ROR_abs_X | OP::ROR_zpg | OP::ROR_zpg_X => {
                let carry = self.get_flag_carry();
                let callback = |x| (x >> 1) | ((carry as u8) << 7);
                if let Some((value, result)) = match OP::from(op) {
                    OP::ROR_A => self.acc(memory, emulator_cycle, callback),
                    OP::ROR_abs => self.abs_rmw(memory, emulator_cycle, callback),
                    OP::ROR_abs_X => self.absx_rmw(memory, emulator_cycle, callback),
                    OP::ROR_zpg => self.zpg_rmw(memory, emulator_cycle, callback),
                    OP::ROR_zpg_X | _ => self.zpgx_rmw(memory, emulator_cycle, callback),
                } {
                    self.set_flag_carry(value & 0b0000_0001 != 0);
                    self.set_flag_zero(result == 0);
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                }
            }

            OP::RRA_X_ind => todo!("{:#04X}", op),
            OP::RRA_abs => todo!("{:#04X}", op),
            OP::RRA_abs_X => todo!("{:#04X}", op),
            OP::RRA_abs_Y => todo!("{:#04X}", op),
            OP::RRA_ind_Y => todo!("{:#04X}", op),
            OP::RRA_zpg => todo!("{:#04X}", op),
            OP::RRA_zpg_X => todo!("{:#04X}", op),

            OP::RTI_impl => todo!("{:#04X}", op),

            OP::RTS_impl => todo!("{:#04X}", op),

            OP::SAX_X_ind => todo!("{:#04X}", op),
            OP::SAX_abs => todo!("{:#04X}", op),
            OP::SAX_zpg => todo!("{:#04X}", op),
            OP::SAX_zpg_Y => todo!("{:#04X}", op),

            OP::SBC_X_ind
            | OP::SBC_abs
            | OP::SBC_abs_X
            | OP::SBC_abs_Y
            | OP::SBC_imm
            | OP::SBC_ind_Y
            | OP::SBC_zpg
            | OP::SBC_zpg_X => {
                let offset: u8 = if self.get_flag_carry() { 0 } else { 1 };
                let callback = |acc, x| acc - x - offset;
                let register = self.accumulator;
                if let Some((value, result)) = match OP::from(op) {
                    OP::SBC_X_ind => self.xind_r(memory, emulator_cycle, register, callback),
                    OP::SBC_abs => self.abs_r(memory, emulator_cycle, register, callback),
                    OP::SBC_abs_X => self.absx_r(memory, emulator_cycle, register, callback),
                    OP::SBC_abs_Y => self.absy_r(memory, emulator_cycle, register, callback),
                    OP::SBC_imm => self.imm_r(memory, emulator_cycle, register, callback),
                    OP::SBC_ind_Y => self.indy_r(memory, emulator_cycle, register, callback),
                    OP::SBC_zpg => self.zpg_r(memory, emulator_cycle, register, callback),
                    OP::SBC_zpg_X | _ => self.zpgx_r(memory, emulator_cycle, register, callback),
                } {
                    self.set_flag_carry(result > register);
                    self.set_flag_zero(result == 0);
                    self.set_flag_overflow(
                        (result ^ register) & (result ^ !value) & 0b1000_0000 != 0,
                    );
                    self.set_flag_negative(result & 0b1000_0000 != 0);
                    self.accumulator = result;
                }
            }

            OP::SBX_imm => todo!("{:#04X}", op),

            OP::SEC_impl => todo!("{:#04X}", op),

            OP::SED_impl => todo!("{:#04X}", op),

            OP::SEI_impl => todo!("{:#04X}", op),

            OP::SHA_abs_Y => todo!("{:#04X}", op),
            OP::SHA_ind_Y => todo!("{:#04X}", op),

            OP::SHX_abs_Y => todo!("{:#04X}", op),

            OP::SHY_abs_X => todo!("{:#04X}", op),

            OP::SLO_X_ind => todo!("{:#04X}", op),
            OP::SLO_abs => todo!("{:#04X}", op),
            OP::SLO_abs_X => todo!("{:#04X}", op),
            OP::SLO_abs_Y => todo!("{:#04X}", op),
            OP::SLO_ind_Y => todo!("{:#04X}", op),
            OP::SLO_zpg => todo!("{:#04X}", op),
            OP::SLO_zpg_X => todo!("{:#04X}", op),

            OP::SRE_X_ind => todo!("{:#04X}", op),
            OP::SRE_abs => todo!("{:#04X}", op),
            OP::SRE_abs_X => todo!("{:#04X}", op),
            OP::SRE_abs_Y => todo!("{:#04X}", op),
            OP::SRE_ind_Y => todo!("{:#04X}", op),
            OP::SRE_zpg => todo!("{:#04X}", op),
            OP::SRE_zpg_X => todo!("{:#04X}", op),

            OP::STA_X_ind => self.xind_w(memory, emulator_cycle, self.accumulator),
            OP::STA_abs => self.abs_w(memory, emulator_cycle, self.accumulator),
            OP::STA_abs_X => self.absx_w(memory, emulator_cycle, self.accumulator),
            OP::STA_abs_Y => self.absy_w(memory, emulator_cycle, self.accumulator),
            OP::STA_ind_Y => self.indy_w(memory, emulator_cycle, self.accumulator),
            OP::STA_zpg => self.zpg_w(memory, emulator_cycle, self.accumulator),
            OP::STA_zpg_X => self.zpgx_w(memory, emulator_cycle, self.accumulator),

            OP::STX_abs => self.abs_w(memory, emulator_cycle, self.index_x),
            OP::STX_zpg => self.zpg_w(memory, emulator_cycle, self.index_x),
            OP::STX_zpg_Y => self.zpgy_w(memory, emulator_cycle, self.index_x),

            OP::STY_abs => self.abs_w(memory, emulator_cycle, self.index_y),
            OP::STY_zpg => self.zpg_w(memory, emulator_cycle, self.index_y),
            OP::STY_zpg_X => self.zpgx_w(memory, emulator_cycle, self.index_y),

            OP::TAS_abs_Y => todo!("{:#04X}", op),

            OP::TAX_impl => {
                if self.cycle == emulator_cycle {
                    self.program_counter -= 1;
                    self.log_instr(memory, OPMode::Impl);
                    self.cycle += 2;
                    return;
                }
                self.index_x = self.accumulator;
                self.set_flag_zero(self.index_x == 0);
                self.set_flag_negative(self.index_x & 0b1000_0000 != 0);
            }

            OP::TAY_impl => {
                if self.cycle == emulator_cycle {
                    self.program_counter -= 1;
                    self.log_instr(memory, OPMode::Impl);
                    self.cycle += 2;
                    return;
                }
                self.index_y = self.accumulator;
                self.set_flag_zero(self.index_y == 0);
                self.set_flag_negative(self.index_y & 0b1000_0000 != 0);
            }

            OP::TSX_impl => {
                if self.cycle == emulator_cycle {
                    self.program_counter -= 1;
                    self.log_instr(memory, OPMode::Impl);
                    self.cycle += 2;
                    return;
                }
                self.index_x = self.stack_pointer;
                self.set_flag_zero(self.index_x == 0);
                self.set_flag_negative(self.index_x & 0b1000_0000 != 0);
            }

            OP::TXA_impl => {
                if self.cycle == emulator_cycle {
                    self.program_counter -= 1;
                    self.log_instr(memory, OPMode::Impl);
                    self.cycle += 2;
                    return;
                }
                self.accumulator = self.index_x;
                self.set_flag_zero(self.accumulator == 0);
                self.set_flag_negative(self.accumulator & 0b1000_0000 != 0);
            }

            OP::TXS_impl => {
                if self.cycle == emulator_cycle {
                    self.program_counter -= 1;
                    self.log_instr(memory, OPMode::Impl);
                    self.cycle += 2;
                    return;
                }
                self.stack_pointer = self.index_x;
                self.set_flag_zero(self.stack_pointer == 0);
                self.set_flag_negative(self.stack_pointer & 0b1000_0000 != 0);
            }

            OP::TYA_impl => {
                if self.cycle == emulator_cycle {
                    self.program_counter -= 1;
                    self.log_instr(memory, OPMode::Impl);
                    self.cycle += 2;
                    return;
                }
                self.accumulator = self.index_y;
                self.set_flag_zero(self.accumulator == 0);
                self.set_flag_negative(self.accumulator & 0b1000_0000 != 0);
            }

            OP::USBC_imm => todo!("{:#04X}", op),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::rom_reader;

    use super::*;

    #[test]
    fn opcodes_inc() {
        let file = rom_reader::compile_and_read_file("./assets/tests/inc.nes");
        let mut memory = Memory {
            ram: vec![0; 0x800],
            ppu_registers: [0; 8],
            apu_io: [0; 32],
            prg_rom: file.prg_rom,
            chr_rom: file.chr_rom,
        };

        let mut cpu = CPU::new(&memory, "log_inc.txt");

        assert!(memory.get(0x02) == 0 as u8);

        for cycle in 7..(7 + 10) {
            cpu.cycle(&mut memory, cycle);
        }

        assert!(memory.get(0x02) == 2 as u8);
    }

    #[test]
    fn opcodes_lda() {
        let file = rom_reader::compile_and_read_file("./assets/tests/LDA.nes");
        let mut memory = Memory {
            ram: vec![0; 0x800],
            ppu_registers: [0; 8],
            apu_io: [0; 32],
            prg_rom: file.prg_rom,
            chr_rom: file.chr_rom,
        };

        let mut cpu = CPU::new(&memory, "log_lda.txt");

        assert!(cpu.accumulator == 0);
        cpu.cycle(&mut memory, 7);
        cpu.cycle(&mut memory, 8);
        assert!(cpu.accumulator == 1);
        cpu.cycle(&mut memory, 9);
        cpu.cycle(&mut memory, 10);
        cpu.cycle(&mut memory, 11);
        cpu.cycle(&mut memory, 12);
        assert!(cpu.accumulator == 2);
        cpu.cycle(&mut memory, 13);
        cpu.cycle(&mut memory, 14);
        cpu.cycle(&mut memory, 15);
        cpu.cycle(&mut memory, 16);
        assert!(cpu.accumulator == 3);
    }
}
