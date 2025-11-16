#[allow(non_camel_case_types)]
pub struct iNES_header {
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
}

#[allow(non_camel_case_types)]
pub struct iNES {
    pub header: iNES_header,
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
    let ines = iNES {
        prg_rom: vec![0; 16384 * ines_header.prg_rom_size as usize],
        chr_rom: vec![0; 8192 * ines_header.chr_rom_size as usize],
        header: ines_header,
    };

    ines
}
