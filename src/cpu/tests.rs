use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use crate::{Memory, cpu::CPU, rom_reader};

// #[test]
// fn opcodes_inc() {
//     let file = rom_reader::compile_and_read_file("./assets/tests/inc.nes");
//     let mut memory = Memory {
//         ram: vec![0; 0x800],
//         ppu_registers: [0; 8],
//         apu_io: [0; 32],
//         prg_rom: file.prg_rom,
//         chr_rom: file.chr_rom,
//     };

//     let mut cpu = CPU::new(&memory, "log_inc.txt");

//     assert!(memory.get(0x02) == 0 as u8);

//     for cycle in 7..(7 + 10) {
//         cpu.cycle(&mut memory, cycle);
//     }

//     assert!(memory.get(0x02) == 2 as u8);
// }

// #[test]
// fn opcodes_lda() {
//     let file = rom_reader::compile_and_read_file("./assets/tests/LDA.nes");
//     let mut memory = Memory {
//         ram: vec![0; 0x800],
//         ppu_registers: [0; 8],
//         apu_io: [0; 32],
//         prg_rom: file.prg_rom,
//         chr_rom: file.chr_rom,
//     };

//     let mut cpu = CPU::new(&memory, "log_lda.txt");

//     assert!(cpu.accumulator == 0);
//     cpu.cycle(&mut memory, 7);
//     cpu.cycle(&mut memory, 8);
//     assert!(cpu.accumulator == 1);
//     cpu.cycle(&mut memory, 9);
//     cpu.cycle(&mut memory, 10);
//     cpu.cycle(&mut memory, 11);
//     cpu.cycle(&mut memory, 12);
//     assert!(cpu.accumulator == 2);
//     cpu.cycle(&mut memory, 13);
//     cpu.cycle(&mut memory, 14);
//     cpu.cycle(&mut memory, 15);
//     cpu.cycle(&mut memory, 16);
//     assert!(cpu.accumulator == 3);
// }

#[test]
fn cpu_full() {
    let file = rom_reader::read_file("./assets/tests/nestest.nes");
    let mut memory = Memory {
        ram: vec![0; 0x800],
        ppu_registers: [0; 8],
        apu_io: [0; 32],
        prg_rom: file.prg_rom,
        chr_rom: file.chr_rom,
    };

    let mut cpu = CPU::new(&memory, "test-cpu_full.log");

    cpu.program_counter = 0xC000;
    let mut emulator_cycle = 7;
    loop {
        if let Err(e) = cpu.cycle(&mut memory, emulator_cycle) {
            println!("CPU crashed with: {e}");
            break;
        }
        emulator_cycle += 1;
    }

    let mut cpu_log = BufReader::new(File::open("./logs/test-cpu_full.log").unwrap());
    let mut reference_log = BufReader::new(File::open("./logs/nestest.log").unwrap());

    let mut line_number = 1;
    loop {
        let mut cpu_log_line = String::new();
        if matches!(cpu_log.read_line(&mut cpu_log_line), Ok(0)) {
            break;
        }

        let mut reference_log_line = String::new();
        if matches!(reference_log.read_line(&mut reference_log_line), Ok(0)) {
            break;
        }

        cpu_log_line = cpu_log_line.trim().to_string();
        reference_log_line = reference_log_line.trim().to_string();

        // Program counter, bytes and mnemonic
        assert!(
            cpu_log_line[0..14] == reference_log_line[0..14],
            "\nLine: {}\n{}\n{}",
            line_number,
            format!(
                "{} {} {} {}",
                "\x1b[31m",
                &cpu_log_line[..14],
                "\x1b[0m",
                &cpu_log_line[14..]
            ),
            format!(
                "{} {} {} {}",
                "\x1b[32m",
                &reference_log_line[0..14],
                "\x1b[0m",
                &reference_log_line[14..]
            ),
        );

        // Accumulator
        assert!(
            cpu_log_line[50..52] == reference_log_line[50..52],
            "\nLine: {}\n{}\n{}",
            line_number,
            format!(
                "{} {} {} {} {}",
                &cpu_log_line[0..50],
                "\x1b[31m",
                &cpu_log_line[50..52],
                "\x1b[0m",
                &cpu_log_line[52..]
            ),
            format!(
                "{} {} {} {} {}",
                &reference_log_line[0..50],
                "\x1b[32m",
                &reference_log_line[50..52],
                "\x1b[0m",
                &reference_log_line[52..]
            ),
        );

        // X
        assert!(
            cpu_log_line[55..57] == reference_log_line[55..57],
            "\nLine: {}\n{}\n{}",
            line_number,
            format!(
                "{} {} {} {} {}",
                &cpu_log_line[0..55],
                "\x1b[31m",
                &cpu_log_line[55..57],
                "\x1b[0m",
                &cpu_log_line[57..]
            ),
            format!(
                "{} {} {} {} {}",
                &reference_log_line[0..55],
                "\x1b[32m",
                &reference_log_line[55..57],
                "\x1b[0m",
                &reference_log_line[57..]
            ),
        );

        // Y
        assert!(
            cpu_log_line[60..62] == reference_log_line[60..62],
            "\nLine: {}\n{}\n{}",
            line_number,
            format!(
                "{} {} {} {} {}",
                &cpu_log_line[0..60],
                "\x1b[31m",
                &cpu_log_line[60..62],
                "\x1b[0m",
                &cpu_log_line[62..]
            ),
            format!(
                "{} {} {} {} {}",
                &reference_log_line[0..60],
                "\x1b[32m",
                &reference_log_line[60..62],
                "\x1b[0m",
                &reference_log_line[62..]
            ),
        );

        // Flags
        assert!(
            cpu_log_line[65..67] == reference_log_line[65..67],
            "Line: {}\n{}\n{}\n{}\n{}",
            line_number,
            format!(
                "{} {} {} {} {}",
                &cpu_log_line[0..65],
                "\x1b[31m",
                &cpu_log_line[65..67],
                "\x1b[0m",
                &cpu_log_line[67..]
            ),
            format!(
                "NV1BDIZC\n{:08b}",
                u8::from_str_radix(&cpu_log_line[65..67], 16).unwrap()
            ),
            format!(
                "{} {} {} {} {}",
                &reference_log_line[0..65],
                "\x1b[32m",
                &reference_log_line[65..67],
                "\x1b[0m",
                &reference_log_line[67..]
            ),
            format!(
                "NV1BDIZC\n{:08b}",
                u8::from_str_radix(&reference_log_line[65..67], 16).unwrap()
            ),
        );

        // Stack Pointer
        assert!(
            cpu_log_line[71..73] == reference_log_line[71..73],
            "Line: {}\n{}\n{}",
            line_number,
            format!(
                "{} {} {} {} {}",
                &cpu_log_line[0..71],
                "\x1b[31m",
                &cpu_log_line[71..73],
                "\x1b[0m",
                &cpu_log_line[73..]
            ),
            format!(
                "{} {} {} {} {}",
                &reference_log_line[0..71],
                "\x1b[32m",
                &reference_log_line[71..73],
                "\x1b[0m",
                &reference_log_line[73..]
            ),
        );

        // Cycles
        assert!(
            cpu_log_line[90..] == reference_log_line[90..],
            "Line: {}\n{}\n{}",
            line_number,
            format!(
                "{} {} {} {}",
                &cpu_log_line[..90],
                "\x1b[31m",
                &cpu_log_line[90..],
                "\x1b[0m",
            ),
            format!(
                "{} {} {} {}",
                &reference_log_line[..90],
                "\x1b[32m",
                &reference_log_line[90..],
                "\x1b[0m",
            ),
        );

        line_number += 1;
    }
}
