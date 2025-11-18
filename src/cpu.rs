mod opcodes;
use std::fs::{File, OpenOptions};
use std::io::Write;

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
    pub fn new(memory: &Memory) -> CPU {
        let log_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("log.txt")
            .unwrap();

        CPU {
            accumulator: 0,
            index_x: 0,
            index_y: 0,
            program_counter: u16::from_le_bytes([memory.get(0xFFFC), memory.get(0xFFFD)]),
            stack_pointer: 0xFD,
            status_register: 0b0010_0000,
            cycle: 0,
            log_file,
        }
    }

    fn get_flag_carry(&self) -> bool {
        self.status_register & 0b1 == 0b1
    }
    fn set_flag_carry(&mut self) {
        self.status_register |= 0b1;
    }
    fn reset_flag_carry(&mut self) {
        self.status_register &= 0b1111_1110;
    }

    fn get_flag_zero(&self) -> bool {
        self.status_register & 0b10 == 0b10
    }
    fn set_flag_zero(&mut self) {
        self.status_register |= 0b10;
    }
    fn reset_flag_zero(&mut self) {
        self.status_register &= 0b1111_1101;
    }

    fn get_flag_interrupt_disable(&self) -> bool {
        self.status_register & 0b100 == 0b100
    }
    fn set_flag_interrupt_disable(&mut self) {
        self.status_register |= 0b100;
    }
    fn reset_flag_interrupt_disable(&mut self) {
        self.status_register &= 0b1111_1011;
    }

    fn get_flag_decimal(&self) -> bool {
        self.status_register & 0b1000 == 0b1000
    }
    fn set_flag_decimal(&mut self) {
        self.status_register |= 0b1000;
    }
    fn reset_flag_decimal(&mut self) {
        self.status_register &= 0b1111_0111;
    }

    fn get_flag_b(&self) -> bool {
        self.status_register & 0b1000_0 == 0b1000_0
    }
    fn set_flag_b(&mut self) {
        self.status_register |= 0b1000_0;
    }
    fn reset_flag_b(&mut self) {
        self.status_register &= 0b1110_1111;
    }

    fn get_flag_overflow(&self) -> bool {
        self.status_register & 0b1000_00 == 0b1000_000
    }
    fn set_flag_overflow(&mut self) {
        self.status_register |= 0b1000_000;
    }
    fn reset_flag_overflow(&mut self) {
        self.status_register &= 0b1011_1111;
    }

    fn get_flag_negative(&self) -> bool {
        self.status_register & 0b1000_0000 == 0b1000_0000
    }
    fn set_flag_negative(&mut self) {
        self.status_register |= 0b1000_0000;
    }
    fn reset_flag_negative(&mut self) {
        self.status_register &= 0b0111_1111;
    }

    fn log_instr(&mut self, bytes: Vec<u8>, mode: OPMode, name: &str) {
        let mut line = String::from("");
        line += &format!("{:04X}  ", self.program_counter);
        for byte in bytes.iter() {
            line += &format!("{:02X} ", byte);
        }
        for _ in 0..(3 - bytes.len()) {
            line += "   ";
        }

        line += &format!("{} ", name);
        line += &format!(
            "{}",
            match mode {
                OPMode::A => String::from("A"),
                OPMode::Abs => format!("${:02X}{:02X}", bytes[2], bytes[1]),
                OPMode::AbsX => format!("${:02X}{:02X}, X", bytes[2], bytes[1]),
                OPMode::AbsY => format!("${:02X}{:02X}, Y", bytes[2], bytes[1]),
                OPMode::Imm => format!("#${:02X}", bytes[1]),
                OPMode::Impl => String::from(""),
                OPMode::Ind => format!("(${:02X}{:02X})", bytes[2], bytes[1]),
                OPMode::XInd => format!("(${:02X},X)", bytes[1]),
                OPMode::IndY => format!("(${:02X}),Y", bytes[1]),
                OPMode::Rel => format!(
                    "$({:04X})",
                    (self.program_counter as i32 + 2 as i32 + (bytes[1] as i8) as i32) as u16
                ),
                OPMode::Zpg => format!("${:02X}", bytes[1]),
                OPMode::ZpgX => format!("${:02X},X", bytes[1]),
                OPMode::ZpgY => format!("${:02X},Y", bytes[1]),
            }
        );

        while line.len() < 48 {
            line += " ";
        }

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
            self.cycle += 6;
            self.program_counter -= 1;
            return None;
        }

        let op = memory.get(self.program_counter);

        let address_lb = memory.get(self.program_counter);
        self.program_counter += 1;
        let address_hb = memory.get(self.program_counter);
        self.program_counter += 1;

        let address = u16::from_le_bytes([address_lb, address_hb]);

        let value = memory.get(address);
        let result = callback(value);

        memory.set(address, result);

        self.log_instr(
            vec![op, address_lb, address_hb],
            OPMode::Abs,
            &OP::from(op).to_string(),
        );

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
            self.cycle += 7;
            self.program_counter -= 1;
            return None;
        }

        let op = memory.get(self.program_counter);

        let address_lb = memory.get(self.program_counter);
        self.program_counter += 1;
        let address_hb = memory.get(self.program_counter);
        self.program_counter += 1;

        let mut address = u16::from_le_bytes([address_lb, address_hb]);
        address += self.index_x as u16;

        let value = memory.get(address);
        let result = callback(value);

        memory.set(address, result);

        self.log_instr(
            vec![op, address_lb, address_hb],
            OPMode::AbsX,
            &OP::from(op).to_string(),
        );

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
            self.cycle += 5;
            self.program_counter -= 1;
            return None;
        }

        let op = memory.get(self.program_counter);

        let address = memory.get(self.program_counter);
        self.program_counter += 1;

        let value = memory.get(address.into());
        let result = callback(value);

        memory.set(address.into(), result);

        self.log_instr(vec![op, address], OPMode::Zpg, &OP::from(op).to_string());

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
            self.cycle += 6;
            self.program_counter -= 1;
            return None;
        }

        let op = memory.get(self.program_counter);

        let mut address = memory.get(self.program_counter);
        self.program_counter += 1;

        address += self.index_x;

        let value = memory.get(address.into());
        let result = callback(value);

        memory.set(address.into(), result);

        self.log_instr(vec![op, address], OPMode::ZpgX, &OP::from(op).to_string());

        Some((value, result))
    }

    fn acc_w<F>(
        &mut self,
        memory: &mut Memory,
        emulator_cycle: u64,
        callback: F,
    ) -> Option<(u8, u8)>
    where
        F: Fn(u8) -> u8,
    {
        if self.cycle == emulator_cycle {
            self.cycle += 2;
            self.program_counter -= 1;
            return None;
        }

        let op = memory.get(self.program_counter);

        let value = self.accumulator;
        let result = callback(value);

        self.accumulator = result;

        self.log_instr(vec![op], OPMode::A, &OP::from(op).to_string());

        Some((value, result))
    }

    pub fn cycle(&mut self, memory: &mut Memory, emulator_cycle: u64) {
        if self.cycle.saturating_sub(1) > emulator_cycle {
            return;
        }

        let op = memory.get(self.program_counter);
        self.program_counter += 1;

        match OP::from(op) {
            OP::ADC_X_ind => todo!("{:#04X}", op),
            OP::ADC_abs => todo!("{:#04X}", op),
            OP::ADC_abs_X => todo!("{:#04X}", op),
            OP::ADC_abs_Y => todo!("{:#04X}", op),
            OP::ADC_imm => todo!("{:#04X}", op),
            OP::ADC_ind_Y => todo!("{:#04X}", op),
            OP::ADC_zpg => todo!("{:#04X}", op),
            OP::ADC_zpg_X => todo!("{:#04X}", op),

            OP::ALR_imm => todo!("{:#04X}", op),

            OP::ANC_imm_0x0b => todo!("{:#04X}", op),
            OP::ANC_imm_0x2b => todo!("{:#04X}", op),

            OP::AND_X_ind => todo!("{:#04X}", op),
            OP::AND_abs => todo!("{:#04X}", op),
            OP::AND_abs_X => todo!("{:#04X}", op),
            OP::AND_abs_Y => todo!("{:#04X}", op),
            OP::AND_imm => todo!("{:#04X}", op),
            OP::AND_ind_Y => todo!("{:#04X}", op),
            OP::AND_zpg => todo!("{:#04X}", op),
            OP::AND_zpg_X => todo!("{:#04X}", op),

            OP::ANE_imm => todo!("{:#04X}", op),

            OP::ARR_imm => todo!("{:#04X}", op),

            OP::ASL_A | OP::ASL_abs | OP::ASL_abs_X | OP::ASL_zpg | OP::ASL_zpg_X => {
                if let Some((value, result)) = match OP::from(op) {
                    OP::ASL_A => self.acc_w(memory, emulator_cycle, |x| x << 1),
                    OP::ASL_abs => self.abs_rmw(memory, emulator_cycle, |x| x << 1),
                    OP::ASL_abs_X => self.absx_rmw(memory, emulator_cycle, |x| x << 1),
                    OP::ASL_zpg => self.zpg_rmw(memory, emulator_cycle, |x| x << 1),
                    OP::ASL_zpg_X | _ => self.zpgx_rmw(memory, emulator_cycle, |x| x << 1),
                } {
                    if value & 0b1000_0000 == 1 {
                        self.set_flag_carry();
                    } else {
                        self.reset_flag_carry();
                    }

                    if result == 0 {
                        self.set_flag_zero();
                    } else {
                        self.reset_flag_zero();
                    }

                    if result & 0b1000_0000 == 1 {
                        self.set_flag_negative();
                    } else {
                        self.reset_flag_negative();
                    }
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
                if let Some((_, result)) = match OP::from(op) {
                    OP::INC_abs => self.abs_rmw(memory, emulator_cycle, |x| x - 1),
                    OP::INC_abs_X => self.absx_rmw(memory, emulator_cycle, |x| x - 1),
                    OP::INC_zpg => self.zpg_rmw(memory, emulator_cycle, |x| x - 1),
                    OP::INC_zpg_X | _ => self.zpgx_rmw(memory, emulator_cycle, |x| x - 1),
                } {
                    if result == 0 {
                        self.set_flag_zero();
                    } else {
                        self.reset_flag_zero();
                    }

                    if result & 0b1000_0000 == 1 {
                        self.set_flag_negative();
                    } else {
                        self.reset_flag_negative();
                    }
                }
            }

            OP::DEX_impl => todo!("{:#04X}", op),

            OP::DEY_impl => todo!("{:#04X}", op),

            OP::EOR_X_ind => todo!("{:#04X}", op),
            OP::EOR_abs => todo!("{:#04X}", op),
            OP::EOR_abs_X => todo!("{:#04X}", op),
            OP::EOR_abs_Y => todo!("{:#04X}", op),
            OP::EOR_imm => todo!("{:#04X}", op),
            OP::EOR_ind_Y => todo!("{:#04X}", op),
            OP::EOR_zpg => todo!("{:#04X}", op),
            OP::EOR_zpg_X => todo!("{:#04X}", op),

            OP::INC_abs | OP::INC_abs_X | OP::INC_zpg | OP::INC_zpg_X => {
                if let Some((_, result)) = match OP::from(op) {
                    OP::INC_abs => self.abs_rmw(memory, emulator_cycle, |x| x + 1),
                    OP::INC_abs_X => self.absx_rmw(memory, emulator_cycle, |x| x + 1),
                    OP::INC_zpg => self.zpg_rmw(memory, emulator_cycle, |x| x + 1),
                    OP::INC_zpg_X | _ => self.zpgx_rmw(memory, emulator_cycle, |x| x + 1),
                } {
                    if result == 0 {
                        self.set_flag_zero();
                    } else {
                        self.reset_flag_zero();
                    }

                    if result & 0b1000_0000 == 1 {
                        self.set_flag_negative();
                    } else {
                        self.reset_flag_negative();
                    }
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

            OP::JMP_abs => todo!("{:#04X}", op),
            OP::JMP_ind => todo!("{:#04X}", op),

            OP::JSR_abs => todo!("{:#04X}", op),

            OP::LAS_abs_Y => todo!("{:#04X}", op),

            OP::LAX_X_ind => todo!("{:#04X}", op),
            OP::LAX_abs => todo!("{:#04X}", op),
            OP::LAX_abs_Y => todo!("{:#04X}", op),
            OP::LAX_ind_Y => todo!("{:#04X}", op),
            OP::LAX_zpg => todo!("{:#04X}", op),
            OP::LAX_zpg_Y => todo!("{:#04X}", op),

            OP::LDA_X_ind => todo!("{:#04X}", op),
            OP::LDA_abs => todo!("{:#04X}", op),
            OP::LDA_abs_X => todo!("{:#04X}", op),
            OP::LDA_abs_Y => todo!("{:#04X}", op),
            OP::LDA_imm => todo!("{:#04X}", op),
            OP::LDA_ind_Y => todo!("{:#04X}", op),
            OP::LDA_zpg => todo!("{:#04X}", op),
            OP::LDA_zpg_X => todo!("{:#04X}", op),

            OP::LDX_abs => todo!("{:#04X}", op),
            OP::LDX_abs_Y => todo!("{:#04X}", op),
            OP::LDX_imm => todo!("{:#04X}", op),
            OP::LDX_zpg => todo!("{:#04X}", op),
            OP::LDX_zpg_Y => todo!("{:#04X}", op),

            OP::LDY_abs => todo!("{:#04X}", op),
            OP::LDY_abs_X => todo!("{:#04X}", op),
            OP::LDY_imm => todo!("{:#04X}", op),
            OP::LDY_zpg => todo!("{:#04X}", op),
            OP::LDY_zpg_X => todo!("{:#04X}", op),

            OP::LSR_A | OP::LSR_abs | OP::LSR_abs_X | OP::LSR_zpg | OP::LSR_zpg_X => {
                if let Some((value, result)) = match OP::from(op) {
                    OP::LSR_A => self.acc_w(memory, emulator_cycle, |x| x >> 1),
                    OP::LSR_abs => self.abs_rmw(memory, emulator_cycle, |x| x >> 1),
                    OP::LSR_abs_X => self.absx_rmw(memory, emulator_cycle, |x| x >> 1),
                    OP::LSR_zpg => self.zpg_rmw(memory, emulator_cycle, |x| x >> 1),
                    OP::LSR_zpg_X | _ => self.zpgx_rmw(memory, emulator_cycle, |x| x >> 1),
                } {
                    if value & 0b0000_0001 == 1 {
                        self.set_flag_carry();
                    } else {
                        self.reset_flag_carry();
                    }

                    if result == 0 {
                        self.set_flag_zero();
                    } else {
                        self.reset_flag_zero();
                    }

                    self.reset_flag_negative();
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

            OP::ORA_X_ind => todo!("{:#04X}", op),
            OP::ORA_abs => todo!("{:#04X}", op),
            OP::ORA_abs_X => todo!("{:#04X}", op),
            OP::ORA_abs_Y => todo!("{:#04X}", op),
            OP::ORA_imm => todo!("{:#04X}", op),
            OP::ORA_ind_Y => todo!("{:#04X}", op),
            OP::ORA_zpg => todo!("{:#04X}", op),
            OP::ORA_zpg_X => todo!("{:#04X}", op),

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
                if let Some((value, result)) = match OP::from(op) {
                    OP::ROL_A => self.acc_w(memory, emulator_cycle, |x| (x << 1) | carry as u8),
                    OP::ROL_abs => self.abs_rmw(memory, emulator_cycle, |x| (x << 1) | carry as u8),
                    OP::ROL_abs_X => {
                        self.absx_rmw(memory, emulator_cycle, |x| (x << 1) | carry as u8)
                    }
                    OP::ROL_zpg => self.zpg_rmw(memory, emulator_cycle, |x| (x << 1) | carry as u8),
                    OP::ROL_zpg_X | _ => {
                        self.zpgx_rmw(memory, emulator_cycle, |x| (x << 1) | carry as u8)
                    }
                } {
                    if value & 0b1000_0000 == 1 {
                        self.set_flag_carry();
                    } else {
                        self.reset_flag_carry();
                    }

                    if result == 0 {
                        self.set_flag_zero();
                    } else {
                        self.reset_flag_zero();
                    }

                    if result & 0b1000_0000 == 1 {
                        self.set_flag_negative();
                    } else {
                        self.reset_flag_negative();
                    }
                }
            }

            OP::ROR_A => todo!("{:#04X}", op),
            OP::ROR_abs => todo!("{:#04X}", op),
            OP::ROR_abs_X => todo!("{:#04X}", op),
            OP::ROR_zpg => todo!("{:#04X}", op),
            OP::ROR_zpg_X => todo!("{:#04X}", op),

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

            OP::SBC_X_ind => todo!("{:#04X}", op),
            OP::SBC_abs => todo!("{:#04X}", op),
            OP::SBC_abs_X => todo!("{:#04X}", op),
            OP::SBC_abs_Y => todo!("{:#04X}", op),
            OP::SBC_imm => todo!("{:#04X}", op),
            OP::SBC_ind_Y => todo!("{:#04X}", op),
            OP::SBC_zpg => todo!("{:#04X}", op),
            OP::SBC_zpg_X => todo!("{:#04X}", op),

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

            OP::STA_X_ind => todo!("{:#04X}", op),
            OP::STA_abs => todo!("{:#04X}", op),
            OP::STA_abs_X => todo!("{:#04X}", op),
            OP::STA_abs_Y => todo!("{:#04X}", op),
            OP::STA_ind_Y => todo!("{:#04X}", op),
            OP::STA_zpg => todo!("{:#04X}", op),
            OP::STA_zpg_X => todo!("{:#04X}", op),

            OP::STX_abs => todo!("{:#04X}", op),
            OP::STX_zpg => todo!("{:#04X}", op),
            OP::STX_zpg_Y => todo!("{:#04X}", op),

            OP::STY_abs => todo!("{:#04X}", op),
            OP::STY_zpg => todo!("{:#04X}", op),
            OP::STY_zpg_X => todo!("{:#04X}", op),

            OP::TAS_abs_Y => todo!("{:#04X}", op),

            OP::TAX_impl => todo!("{:#04X}", op),

            OP::TAY_impl => todo!("{:#04X}", op),

            OP::TSX_impl => todo!("{:#04X}", op),

            OP::TXA_impl => todo!("{:#04X}", op),

            OP::TXS_impl => todo!("{:#04X}", op),

            OP::TYA_impl => todo!("{:#04X}", op),

            OP::USBC_imm => todo!("{:#04X}", op),
        }
    }
}

#[cfg(test)]
mod tests {
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

        let mut cpu = CPU::new(&memory);

        println!("$(0x2) = {}", memory.get(0x02));

        cpu.cycle(&mut memory, 0);
        cpu.cycle(&mut memory, 1);
        cpu.cycle(&mut memory, 2);
        cpu.cycle(&mut memory, 3);
        cpu.cycle(&mut memory, 4);
        cpu.cycle(&mut memory, 5);
        cpu.cycle(&mut memory, 6);
        cpu.cycle(&mut memory, 7);
        cpu.cycle(&mut memory, 8);
        cpu.cycle(&mut memory, 9);

        println!("$(0x2) = {}", memory.get(0x02));
    }
}
