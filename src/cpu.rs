mod opcodes;
use opcodes::OP;

pub struct CPU {
    accumulator: u8,
    index_x: u8,
    index_y: u8,
    program_counter: u16,
    stack_pointer: u8,
    status_register: u8,
}

impl CPU {
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

    pub fn execute_instruction(&mut self, op: u8, ram: &mut Vec<u8>) {
        match OP::from(op) {
            OP::ADC_imm => todo!(),
            OP::ADC_zpg => todo!(),
            OP::ADC_zpg_X => todo!(),
            OP::ADC_abs => todo!(),
            OP::ADC_abs_X => todo!(),
            OP::ADC_abs_Y => todo!(),
            OP::ADC_X_ind => todo!(),
            OP::ADC_ind_Y => todo!(),

            OP::BRK_impl => todo!(),
            OP::ORA_X_ind => todo!(),
            OP::JAM_0x2 => todo!(),
            OP::SLO_X_ind => todo!(),
            OP::NOP_zpg_0x4 => todo!(),
            OP::ORA_zpg => todo!(),
            OP::ASL_zpg => todo!(),
            OP::SLO_zpg => todo!(),
            OP::PHP_impl => todo!(),
            OP::ORA_imm => todo!(),
            OP::ASL_A => todo!(),
            OP::ANC_imm_0x0b => todo!(),
            OP::NOP_abs_0xc => todo!(),
            OP::ORA_abs => todo!(),
            OP::ASL_abs => todo!(),
            OP::SLO_abs => todo!(),
            OP::BPL_rel => todo!(),
            OP::ORA_ind_Y => todo!(),
            OP::JAM_0x12 => todo!(),
            OP::SLO_ind_Y => todo!(),
            OP::NOP_zpg_X_0x14 => todo!(),
            OP::ORA_zpg_X => todo!(),
            OP::ASL_zpg_X => todo!(),
            OP::SLO_zpg_X => todo!(),
            OP::CLC_impl => todo!(),
            OP::ORA_abs_Y => todo!(),
            OP::NOP_impl_0x1a => todo!(),
            OP::SLO_abs_Y => todo!(),
            OP::NOP_abs_X_0x1c => todo!(),
            OP::ORA_abs_X => todo!(),
            OP::ASL_abs_X => todo!(),
            OP::SLO_abs_X => todo!(),
            OP::JSR_abs => todo!(),
            OP::AND_X_ind => todo!(),
            OP::JAM_0x22 => todo!(),
            OP::RLA_X_ind => todo!(),
            OP::BIT_zpg => todo!(),
            OP::AND_zpg => todo!(),
            OP::ROL_zpg => todo!(),
            OP::RLA_zpg => todo!(),
            OP::PLP_impl => todo!(),
            OP::AND_imm => todo!(),
            OP::ROL_A => todo!(),
            OP::ANC_imm_0x2b => todo!(),
            OP::BIT_abs => todo!(),
            OP::AND_abs => todo!(),
            OP::ROL_abs => todo!(),
            OP::RLA_abs => todo!(),
            OP::BMI_rel => todo!(),
            OP::AND_ind_Y => todo!(),
            OP::JAM_0x32 => todo!(),
            OP::RLA_ind_Y => todo!(),
            OP::NOP_zpg_X_0x34 => todo!(),
            OP::AND_zpg_X => todo!(),
            OP::ROL_zpg_X => todo!(),
            OP::RLA_zpg_X => todo!(),
            OP::SEC_impl => todo!(),
            OP::AND_abs_Y => todo!(),
            OP::NOP_impl_0x3a => todo!(),
            OP::RLA_abs_Y => todo!(),
            OP::NOP_abs_X_0x3c => todo!(),
            OP::AND_abs_X => todo!(),
            OP::ROL_abs_X => todo!(),
            OP::RLA_abs_X => todo!(),
            OP::RTI_impl => todo!(),
            OP::EOR_X_ind => todo!(),
            OP::JAM_0x42 => todo!(),
            OP::SRE_X_ind => todo!(),
            OP::NOP_zpg_0x44 => todo!(),
            OP::EOR_zpg => todo!(),
            OP::LSR_zpg => todo!(),
            OP::SRE_zpg => todo!(),
            OP::PHA_impl => todo!(),
            OP::EOR_imm => todo!(),
            OP::LSR_A => todo!(),
            OP::ALR_imm => todo!(),
            OP::JMP_abs => todo!(),
            OP::EOR_abs => todo!(),
            OP::LSR_abs => todo!(),
            OP::SRE_abs => todo!(),
            OP::BVC_rel => todo!(),
            OP::EOR_ind_Y => todo!(),
            OP::JAM_0x52 => todo!(),
            OP::SRE_ind_Y => todo!(),
            OP::NOP_zpg_X_0x54 => todo!(),
            OP::EOR_zpg_X => todo!(),
            OP::LSR_zpg_X => todo!(),
            OP::SRE_zpg_X => todo!(),
            OP::CLI_impl => todo!(),
            OP::EOR_abs_Y => todo!(),
            OP::NOP_impl_0x5a => todo!(),
            OP::SRE_abs_Y => todo!(),
            OP::NOP_abs_X_0x5c => todo!(),
            OP::EOR_abs_X => todo!(),
            OP::LSR_abs_X => todo!(),
            OP::SRE_abs_X => todo!(),
            OP::RTS_impl => todo!(),
            OP::JAM_0x62 => todo!(),
            OP::RRA_X_ind => todo!(),
            OP::NOP_zpg_0x64 => todo!(),
            OP::ROR_zpg => todo!(),
            OP::RRA_zpg => todo!(),
            OP::PLA_impl => todo!(),
            OP::ROR_A => todo!(),
            OP::ARR_imm => todo!(),
            OP::JMP_ind => todo!(),
            OP::ROR_abs => todo!(),
            OP::RRA_abs => todo!(),
            OP::BVS_rel => todo!(),
            OP::JAM_0x72 => todo!(),
            OP::RRA_ind_Y => todo!(),
            OP::NOP_zpg_X_0x74 => todo!(),
            OP::ROR_zpg_X => todo!(),
            OP::RRA_zpg_X => todo!(),
            OP::SEI_impl => todo!(),
            OP::NOP_impl_0x7a => todo!(),
            OP::RRA_abs_Y => todo!(),
            OP::NOP_abs_X_0x7c => todo!(),
            OP::ROR_abs_X => todo!(),
            OP::RRA_abs_X => todo!(),
            OP::NOP_imm_0x80 => todo!(),
            OP::STA_X_ind => todo!(),
            OP::NOP_imm_0x82 => todo!(),
            OP::SAX_X_ind => todo!(),
            OP::STY_zpg => todo!(),
            OP::STA_zpg => todo!(),
            OP::STX_zpg => todo!(),
            OP::SAX_zpg => todo!(),
            OP::DEY_impl => todo!(),
            OP::NOP_imm_0x89 => todo!(),
            OP::TXA_impl => todo!(),
            OP::ANE_imm => todo!(),
            OP::STY_abs => todo!(),
            OP::STA_abs => todo!(),
            OP::STX_abs => todo!(),
            OP::SAX_abs => todo!(),
            OP::BCC_rel => todo!(),
            OP::STA_ind_Y => todo!(),
            OP::JAM_0x92 => todo!(),
            OP::SHA_ind_Y => todo!(),
            OP::STY_zpg_X => todo!(),
            OP::STA_zpg_X => todo!(),
            OP::STX_zpg_Y => todo!(),
            OP::SAX_zpg_Y => todo!(),
            OP::TYA_impl => todo!(),
            OP::STA_abs_Y => todo!(),
            OP::TXS_impl => todo!(),
            OP::TAS_abs_Y => todo!(),
            OP::SHY_abs_X => todo!(),
            OP::STA_abs_X => todo!(),
            OP::SHX_abs_Y => todo!(),
            OP::SHA_abs_Y => todo!(),
            OP::LDY_imm => todo!(),
            OP::LDA_X_ind => todo!(),
            OP::LDX_imm => todo!(),
            OP::LAX_X_ind => todo!(),
            OP::LDY_zpg => todo!(),
            OP::LDA_zpg => todo!(),
            OP::LDX_zpg => todo!(),
            OP::LAX_zpg => todo!(),
            OP::TAY_impl => todo!(),
            OP::LDA_imm => todo!(),
            OP::TAX_impl => todo!(),
            OP::LXA_imm => todo!(),
            OP::LDY_abs => todo!(),
            OP::LDA_abs => todo!(),
            OP::LDX_abs => todo!(),
            OP::LAX_abs => todo!(),
            OP::BCS_rel => todo!(),
            OP::LDA_ind_Y => todo!(),
            OP::JAM_0xb2 => todo!(),
            OP::LAX_ind_Y => todo!(),
            OP::LDY_zpg_X => todo!(),
            OP::LDA_zpg_X => todo!(),
            OP::LDX_zpg_Y => todo!(),
            OP::LAX_zpg_Y => todo!(),
            OP::CLV_impl => todo!(),
            OP::LDA_abs_Y => todo!(),
            OP::TSX_impl => todo!(),
            OP::LAS_abs_Y => todo!(),
            OP::LDY_abs_X => todo!(),
            OP::LDA_abs_X => todo!(),
            OP::LDX_abs_Y => todo!(),
            OP::LAX_abs_Y => todo!(),
            OP::CPY_imm => todo!(),
            OP::CMP_X_ind => todo!(),
            OP::NOP_imm_0xc2 => todo!(),
            OP::DCP_X_ind => todo!(),
            OP::CPY_zpg => todo!(),
            OP::CMP_zpg => todo!(),
            OP::DEC_zpg => todo!(),
            OP::DCP_zpg => todo!(),
            OP::INY_impl => todo!(),
            OP::CMP_imm => todo!(),
            OP::DEX_impl => todo!(),
            OP::SBX_imm => todo!(),
            OP::CPY_abs => todo!(),
            OP::CMP_abs => todo!(),
            OP::DEC_abs => todo!(),
            OP::DCP_abs => todo!(),
            OP::BNE_rel => todo!(),
            OP::CMP_ind_Y => todo!(),
            OP::JAM_0xd2 => todo!(),
            OP::DCP_ind_Y => todo!(),
            OP::NOP_zpg_X_0xd4 => todo!(),
            OP::CMP_zpg_X => todo!(),
            OP::DEC_zpg_X => todo!(),
            OP::DCP_zpg_X => todo!(),
            OP::CLD_impl => todo!(),
            OP::CMP_abs_Y => todo!(),
            OP::NOP_impl_0xda => todo!(),
            OP::DCP_abs_Y => todo!(),
            OP::NOP_abs_X_0xdc => todo!(),
            OP::CMP_abs_X => todo!(),
            OP::DEC_abs_X => todo!(),
            OP::DCP_abs_X => todo!(),
            OP::CPX_imm => todo!(),
            OP::SBC_X_ind => todo!(),
            OP::NOP_imm_0xe2 => todo!(),
            OP::ISC_X_ind => todo!(),
            OP::CPX_zpg => todo!(),
            OP::SBC_zpg => todo!(),
            OP::INC_zpg => todo!(),
            OP::ISC_zpg => todo!(),
            OP::INX_impl => todo!(),
            OP::SBC_imm => todo!(),
            OP::NOP_impl_0xea => todo!(),
            OP::USBC_imm => todo!(),
            OP::CPX_abs => todo!(),
            OP::SBC_abs => todo!(),
            OP::INC_abs => todo!(),
            OP::ISC_abs => todo!(),
            OP::BEQ_rel => todo!(),
            OP::SBC_ind_Y => todo!(),
            OP::JAM_0xf2 => todo!(),
            OP::ISC_ind_Y => todo!(),
            OP::NOP_zpg_X_0xf4 => todo!(),
            OP::SBC_zpg_X => todo!(),
            OP::INC_zpg_X => todo!(),
            OP::ISC_zpg_X => todo!(),
            OP::SED_impl => todo!(),
            OP::SBC_abs_Y => todo!(),
            OP::NOP_impl_0xfa => todo!(),
            OP::ISC_abs_Y => todo!(),
            OP::NOP_abs_X_0xfc => todo!(),
            OP::SBC_abs_X => todo!(),
            OP::INC_abs_X => todo!(),
            OP::ISC_abs_X => todo!(),
        }
    }
}
