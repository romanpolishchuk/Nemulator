#[allow(non_camel_case_types)]
pub struct iNES_header {
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
}

#[allow(non_camel_case_types)]
pub struct iNES {
    pub header: iNES_header,
    pub trainer: Vec<u8>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

pub fn read_file(filename: &str) -> iNES {
    let file = std::fs::read(filename).unwrap();
    assert!(file[0..4] == vec!['N' as u8, 'E' as u8, 'S' as u8, 0x1A]);
    let ines_header = iNES_header {
        prg_rom_size: file[4],
        chr_rom_size: file[5],
    };
    let mut pointer = 16;
    let mut trainer = vec![];
    if file[6] & 0b0000_0100 == 1 {
        trainer.copy_from_slice(&file[pointer..pointer + 512]);
        pointer += 512;
    }

    let mut prg_rom = vec![0; 16384 * ines_header.prg_rom_size as usize];
    if ines_header.prg_rom_size > 0 {
        prg_rom
            .copy_from_slice(&file[pointer..pointer + 16384 * ines_header.prg_rom_size as usize]);
        pointer += 16384 * ines_header.prg_rom_size as usize;
    }

    println!("{:X}", prg_rom[16381]);
    println!("{:X}", prg_rom[16380]);

    let mut chr_rom = vec![0; 8192 * ines_header.prg_rom_size as usize];
    if ines_header.chr_rom_size > 0 {
        chr_rom.copy_from_slice(&file[pointer..pointer + 8192 * ines_header.chr_rom_size as usize]);
        pointer += 8192 * ines_header.chr_rom_size as usize;
    }

    let ines = iNES {
        prg_rom: prg_rom,
        chr_rom: chr_rom,
        trainer: trainer,
        header: ines_header,
    };

    ines
}

pub fn compile_and_read_file(filename: &str) -> iNES {
    let file = std::fs::read(filename).unwrap();

    let ines_header = iNES_header {
        prg_rom_size: 1,
        chr_rom_size: 0,
    };

    let mut prg_rom = vec![0; 16384 * ines_header.prg_rom_size as usize];
    prg_rom[0..file.len().min(16384)].copy_from_slice(&file[0..file.len().min(16384)]);

    prg_rom[16380] = 0x00;
    prg_rom[16381] = 0x80;

    let ines = iNES {
        prg_rom: prg_rom,
        chr_rom: vec![],
        trainer: vec![],
        header: ines_header,
    };

    ines
}
