mod opcodes;
use std::fmt::format;
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
    pub fn new() -> CPU {
        let mut log_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("log.txt")
            .unwrap();

        CPU {
            accumulator: 0,
            index_x: 0,
            index_y: 0,
            program_counter: 0xC000,
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

    pub fn cycle(&mut self, memory: &mut Memory, emulator_cycle: u64) {
        if self.cycle > emulator_cycle {
            return;
        }

        let op = memory.get(self.program_counter);
        self.program_counter += 1;
        self.cycle += 1;

        match OP::from(op) {
            OP::ADC_X_ind => todo!(),
            OP::ADC_abs => todo!(),
            OP::ADC_abs_X => todo!(),
            OP::ADC_abs_Y => todo!(),
            OP::ADC_imm => todo!(),
            OP::ADC_ind_Y => todo!(),
            OP::ADC_zpg => todo!(),
            OP::ADC_zpg_X => todo!(),

            OP::ALR_imm => todo!(),

            OP::ANC_imm_0x0b => todo!(),
            OP::ANC_imm_0x2b => todo!(),

            OP::AND_X_ind => todo!(),
            OP::AND_abs => todo!(),
            OP::AND_abs_X => todo!(),
            OP::AND_abs_Y => todo!(),
            OP::AND_imm => todo!(),
            OP::AND_ind_Y => todo!(),
            OP::AND_zpg => todo!(),
            OP::AND_zpg_X => todo!(),

            OP::ANE_imm => todo!(),

            OP::ARR_imm => todo!(),

            OP::ASL_A => todo!(),
            OP::ASL_abs => todo!(),
            OP::ASL_abs_X => todo!(),
            OP::ASL_zpg => todo!(),
            OP::ASL_zpg_X => todo!(),

            OP::BCC_rel => todo!(),

            OP::BCS_rel => todo!(),

            OP::BEQ_rel => todo!(),

            OP::BIT_abs => todo!(),
            OP::BIT_zpg => todo!(),

            OP::BMI_rel => todo!(),

            OP::BNE_rel => todo!(),

            OP::BPL_rel => todo!(),

            OP::BRK_impl => todo!(),

            OP::BVC_rel => todo!(),

            OP::BVS_rel => todo!(),

            OP::CLC_impl => todo!(),

            OP::CLD_impl => todo!(),

            OP::CLI_impl => todo!(),

            OP::CLV_impl => todo!(),

            OP::CMP_X_ind => todo!(),
            OP::CMP_abs => todo!(),
            OP::CMP_abs_X => todo!(),
            OP::CMP_abs_Y => todo!(),
            OP::CMP_imm => todo!(),
            OP::CMP_ind_Y => todo!(),
            OP::CMP_zpg => todo!(),
            OP::CMP_zpg_X => todo!(),

            OP::CPX_abs => todo!(),
            OP::CPX_imm => todo!(),
            OP::CPX_zpg => todo!(),

            OP::CPY_abs => todo!(),
            OP::CPY_imm => todo!(),
            OP::CPY_zpg => todo!(),

            OP::DCP_X_ind => todo!(),
            OP::DCP_abs => todo!(),
            OP::DCP_abs_X => todo!(),
            OP::DCP_abs_Y => todo!(),
            OP::DCP_ind_Y => todo!(),
            OP::DCP_zpg => todo!(),
            OP::DCP_zpg_X => todo!(),

            OP::DEC_abs => todo!(),
            OP::DEC_abs_X => todo!(),
            OP::DEC_zpg => todo!(),
            OP::DEC_zpg_X => todo!(),

            OP::DEX_impl => todo!(),

            OP::DEY_impl => todo!(),

            OP::EOR_X_ind => todo!(),
            OP::EOR_abs => todo!(),
            OP::EOR_abs_X => todo!(),
            OP::EOR_abs_Y => todo!(),
            OP::EOR_imm => todo!(),
            OP::EOR_ind_Y => todo!(),
            OP::EOR_zpg => todo!(),
            OP::EOR_zpg_X => todo!(),

            OP::INC_abs => {
                let address_lb = memory.get(self.program_counter);
                self.program_counter += 1;
                self.cycle += 1;

                let address_hb = memory.get(self.program_counter);
                self.program_counter += 1;
                self.cycle += 1;

                let address = u16::from_le_bytes([address_lb, address_hb]);

                let mut value = memory.get(address);
                self.cycle += 1;

                //memory.set(address, value);
                value += 1;
                self.cycle += 1;

                memory.set(address, value);
                self.cycle += 1;
            }
            OP::INC_abs_X => {
                let mut address_lb = memory.get(self.program_counter);
                self.program_counter += 1;
                self.cycle += 1;

                let mut address_hb = memory.get(self.program_counter);
                self.program_counter += 1;
                let (address_lb, is_overflow) = address_lb.overflowing_add(self.index_x);
                self.cycle += 1;

                if is_overflow {
                    address_hb += 1;
                }
                self.cycle += 1;

                let mut address = u16::from_le_bytes([address_lb, address_hb]);

                let mut value = memory.get(address);
                self.cycle += 1;

                //memory.set(address, value);
                value += 1;
                self.cycle += 1;

                memory.set(address, value);
                self.cycle += 1;
            }
            OP::INC_zpg => {
                let address = memory.get(self.program_counter);
                self.program_counter += 1;
                self.cycle += 1;

                let mut value = memory.get(address.into());
                self.cycle += 1;

                //memory.set(address.into(), value);
                value += 1;
                self.cycle += 1;

                memory.set(address.into(), value);
                self.cycle += 1;
            }
            OP::INC_zpg_X => {
                let mut address = memory.get(self.program_counter);
                self.program_counter += 1;
                self.cycle += 1;

                address += self.index_x;
                self.cycle += 1;

                let mut value = memory.get(address.into());
                self.cycle += 1;

                //memory.set(address.into(), value);
                value += 1;
                self.cycle += 1;

                memory.set(address.into(), value);
                self.cycle += 1;
            }

            OP::INX_impl => todo!(),

            OP::INY_impl => todo!(),

            OP::ISC_X_ind => todo!(),
            OP::ISC_abs => todo!(),
            OP::ISC_abs_X => todo!(),
            OP::ISC_abs_Y => todo!(),
            OP::ISC_ind_Y => todo!(),
            OP::ISC_zpg => todo!(),
            OP::ISC_zpg_X => todo!(),

            OP::JAM_0x12 => todo!(),
            OP::JAM_0x2 => todo!(),
            OP::JAM_0x22 => todo!(),
            OP::JAM_0x32 => todo!(),
            OP::JAM_0x42 => todo!(),
            OP::JAM_0x52 => todo!(),
            OP::JAM_0x62 => todo!(),
            OP::JAM_0x72 => todo!(),
            OP::JAM_0x92 => todo!(),
            OP::JAM_0xb2 => todo!(),
            OP::JAM_0xd2 => todo!(),
            OP::JAM_0xf2 => todo!(),

            OP::JMP_abs => todo!(),
            OP::JMP_ind => todo!(),

            OP::JSR_abs => todo!(),

            OP::LAS_abs_Y => todo!(),

            OP::LAX_X_ind => todo!(),
            OP::LAX_abs => todo!(),
            OP::LAX_abs_Y => todo!(),
            OP::LAX_ind_Y => todo!(),
            OP::LAX_zpg => todo!(),
            OP::LAX_zpg_Y => todo!(),

            OP::LDA_X_ind => todo!(),
            OP::LDA_abs => todo!(),
            OP::LDA_abs_X => todo!(),
            OP::LDA_abs_Y => todo!(),
            OP::LDA_imm => todo!(),
            OP::LDA_ind_Y => todo!(),
            OP::LDA_zpg => todo!(),
            OP::LDA_zpg_X => todo!(),

            OP::LDX_abs => todo!(),
            OP::LDX_abs_Y => todo!(),
            OP::LDX_imm => todo!(),
            OP::LDX_zpg => todo!(),
            OP::LDX_zpg_Y => todo!(),

            OP::LDY_abs => todo!(),
            OP::LDY_abs_X => todo!(),
            OP::LDY_imm => todo!(),
            OP::LDY_zpg => todo!(),
            OP::LDY_zpg_X => todo!(),

            OP::LSR_A => todo!(),
            OP::LSR_abs => todo!(),
            OP::LSR_abs_X => todo!(),
            OP::LSR_zpg => todo!(),
            OP::LSR_zpg_X => todo!(),

            OP::LXA_imm => todo!(),

            OP::NOP_abs_0xc => todo!(),
            OP::NOP_abs_X_0x1c => todo!(),
            OP::NOP_abs_X_0x3c => todo!(),
            OP::NOP_abs_X_0x5c => todo!(),
            OP::NOP_abs_X_0x7c => todo!(),
            OP::NOP_abs_X_0xdc => todo!(),
            OP::NOP_abs_X_0xfc => todo!(),
            OP::NOP_imm_0x80 => todo!(),
            OP::NOP_imm_0x82 => todo!(),
            OP::NOP_imm_0x89 => todo!(),
            OP::NOP_imm_0xc2 => todo!(),
            OP::NOP_imm_0xe2 => todo!(),
            OP::NOP_impl_0x1a => todo!(),
            OP::NOP_impl_0x3a => todo!(),
            OP::NOP_impl_0x5a => todo!(),
            OP::NOP_impl_0x7a => todo!(),
            OP::NOP_impl_0xda => todo!(),
            OP::NOP_impl_0xea => todo!(),
            OP::NOP_impl_0xfa => todo!(),
            OP::NOP_zpg_0x4 => todo!(),
            OP::NOP_zpg_0x44 => todo!(),
            OP::NOP_zpg_0x64 => todo!(),
            OP::NOP_zpg_X_0x14 => todo!(),
            OP::NOP_zpg_X_0x34 => todo!(),
            OP::NOP_zpg_X_0x54 => todo!(),
            OP::NOP_zpg_X_0x74 => todo!(),
            OP::NOP_zpg_X_0xd4 => todo!(),
            OP::NOP_zpg_X_0xf4 => todo!(),

            OP::ORA_X_ind => todo!(),
            OP::ORA_abs => todo!(),
            OP::ORA_abs_X => todo!(),
            OP::ORA_abs_Y => todo!(),
            OP::ORA_imm => todo!(),
            OP::ORA_ind_Y => todo!(),
            OP::ORA_zpg => todo!(),
            OP::ORA_zpg_X => todo!(),

            OP::PHA_impl => todo!(),

            OP::PHP_impl => todo!(),

            OP::PLA_impl => todo!(),

            OP::PLP_impl => todo!(),

            OP::RLA_X_ind => todo!(),
            OP::RLA_abs => todo!(),
            OP::RLA_abs_X => todo!(),
            OP::RLA_abs_Y => todo!(),
            OP::RLA_ind_Y => todo!(),
            OP::RLA_zpg => todo!(),
            OP::RLA_zpg_X => todo!(),

            OP::ROL_A => todo!(),
            OP::ROL_abs => todo!(),
            OP::ROL_abs_X => todo!(),
            OP::ROL_zpg => todo!(),
            OP::ROL_zpg_X => todo!(),

            OP::ROR_A => todo!(),
            OP::ROR_abs => todo!(),
            OP::ROR_abs_X => todo!(),
            OP::ROR_zpg => todo!(),
            OP::ROR_zpg_X => todo!(),

            OP::RRA_X_ind => todo!(),
            OP::RRA_abs => todo!(),
            OP::RRA_abs_X => todo!(),
            OP::RRA_abs_Y => todo!(),
            OP::RRA_ind_Y => todo!(),
            OP::RRA_zpg => todo!(),
            OP::RRA_zpg_X => todo!(),

            OP::RTI_impl => todo!(),

            OP::RTS_impl => todo!(),

            OP::SAX_X_ind => todo!(),
            OP::SAX_abs => todo!(),
            OP::SAX_zpg => todo!(),
            OP::SAX_zpg_Y => todo!(),

            OP::SBC_X_ind => todo!(),
            OP::SBC_abs => todo!(),
            OP::SBC_abs_X => todo!(),
            OP::SBC_abs_Y => todo!(),
            OP::SBC_imm => todo!(),
            OP::SBC_ind_Y => todo!(),
            OP::SBC_zpg => todo!(),
            OP::SBC_zpg_X => todo!(),

            OP::SBX_imm => todo!(),

            OP::SEC_impl => todo!(),

            OP::SED_impl => todo!(),

            OP::SEI_impl => todo!(),

            OP::SHA_abs_Y => todo!(),
            OP::SHA_ind_Y => todo!(),

            OP::SHX_abs_Y => todo!(),

            OP::SHY_abs_X => todo!(),

            OP::SLO_X_ind => todo!(),
            OP::SLO_abs => todo!(),
            OP::SLO_abs_X => todo!(),
            OP::SLO_abs_Y => todo!(),
            OP::SLO_ind_Y => todo!(),
            OP::SLO_zpg => todo!(),
            OP::SLO_zpg_X => todo!(),

            OP::SRE_X_ind => todo!(),
            OP::SRE_abs => todo!(),
            OP::SRE_abs_X => todo!(),
            OP::SRE_abs_Y => todo!(),
            OP::SRE_ind_Y => todo!(),
            OP::SRE_zpg => todo!(),
            OP::SRE_zpg_X => todo!(),

            OP::STA_X_ind => todo!(),
            OP::STA_abs => todo!(),
            OP::STA_abs_X => todo!(),
            OP::STA_abs_Y => todo!(),
            OP::STA_ind_Y => todo!(),
            OP::STA_zpg => todo!(),
            OP::STA_zpg_X => todo!(),

            OP::STX_abs => todo!(),
            OP::STX_zpg => todo!(),
            OP::STX_zpg_Y => todo!(),

            OP::STY_abs => todo!(),
            OP::STY_zpg => todo!(),
            OP::STY_zpg_X => todo!(),

            OP::TAS_abs_Y => todo!(),

            OP::TAX_impl => todo!(),

            OP::TAY_impl => todo!(),

            OP::TSX_impl => todo!(),

            OP::TXA_impl => todo!(),

            OP::TXS_impl => todo!(),

            OP::TYA_impl => todo!(),

            OP::USBC_imm => todo!(),
        }
    }
}
