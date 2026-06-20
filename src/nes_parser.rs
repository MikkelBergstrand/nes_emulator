use core::fmt;
use std::{fs, usize};

use std::error::Error;

#[derive(Debug)]
enum NESFileFormat {
    INES,
    NES2,
}

#[derive(Debug)]
enum TimingMode {
    NTSC,
    PAL,
    MultipleRegion,
    Dendy
}

#[derive(Debug)]
pub struct NES2Header {
    pub nametable_arrangement: NametableArrangement,
    pub nvm_present: bool,
    pub trainer: bool,
    pub alt_nametable_layout: bool,
    pub console_type: ConsoleType,
    pub mapper: u16,  
    pub submapper: u8,
    pub prg_rom_size: u16, 
    pub chr_rom_size: u16,
    pub prg_ram_size: u16,
    pub prg_nvram_size: u16,
    pub chr_ram_size: u16,
    pub chr_nvram_size: u16,
    pub timing_mode: TimingMode,
    pub default_expansion_device: u8,
    pub misc_roms: u8,
}

pub struct NESData {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub header: NES2Header,
}

#[derive(Debug)]
pub enum NametableArrangement {
    Vertical,
    Horizontal
}

#[derive(Debug)]
enum ConsoleType {
    NES,
    VsSystem,
    Playchoice10,
    Extended
}


#[derive(Debug)]
pub enum RomError {
    Io(std::io::Error),
    UnrecognizedFormat,
}

impl fmt::Display for RomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RomError::Io(e) => write!(f, "I/O error reading ROM: {e}"),
            RomError::UnrecognizedFormat => write!(f, "unrecognized ROM format"),
        }
    }
}

impl std::error::Error for RomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
       match self {
            RomError::Io(e) => Some(e),
            RomError::UnrecognizedFormat => None
       } 
    }
}


impl From<std::io::Error> for RomError {
    fn from(e: std::io::Error) -> Self {
        RomError::Io(e)
    }
}

pub fn read(filename: &str) -> Result<NESData, RomError> {
    let bytes: Vec<u8> = fs::read(filename)?;

    if bytes[0..4] != [0x4E, 0x45, 0x53, 0x1A] { // 'NES{EOF}' in ASCII
        return Err(RomError::UnrecognizedFormat);
    }


    //if bytes[7] & 0x0C != 0x08 {
    //    return Err(RomError::UnrecognizedFormat);
    //}

    let nes2_header = parse_nes2_header(&bytes);
    dbg!("{}", &nes2_header);
    
    let mut offset: usize = 16; // Length of header

    // ROM Size is in units of 16kB
    let prg_rom_size: usize = nes2_header.prg_rom_size as usize * (1 << 14);
    let prg_rom_data = &bytes.get(offset..(offset+prg_rom_size)).ok_or(RomError::UnrecognizedFormat)?;
    offset += prg_rom_size;

    // Units of 8kB
    let chr_rom_size = nes2_header.chr_rom_size as usize * (1 << 13);
    let chr_rom_data = bytes.get(offset..(offset+chr_rom_size)).ok_or(RomError::UnrecognizedFormat)?;
    // offset += chr_rom_size;

    Ok(NESData {
        header: nes2_header,
        prg_rom: prg_rom_data.to_vec(),
        chr_rom: chr_rom_data.to_vec(),
    })
}

pub fn parse_nes2_header(bytes: &[u8]) -> NES2Header {
    fn bit_to_bool(byte: u8, bit: u8) -> bool { (byte & (1 << bit)) != 0 }
    fn parse_shift_format(shift_count: u8) -> u16 {
        if shift_count == 0 {
            return 0;
        }
        return 64 << shift_count;
    }
    println!("NES 2.0 format recognized");
    
    let mapper: u16 = ((bytes[6] as u16) >> 4) 
        | (( bytes[7] as u16) & 0xF0)
        | (((bytes[8] as u16) & 0x0F) << 4);
    
    let console_type =
        match bytes[7] & 0x03 {
            0 => ConsoleType::NES,
            1 => ConsoleType::VsSystem,
            2 => ConsoleType::Playchoice10,
            3 => ConsoleType::Extended,
            _ => panic!("Error parsing console type")
        };

    let timing_mode = 
        match bytes[12] & 0x03 {
            0 => TimingMode::NTSC,
            1 => TimingMode::PAL,
            2 => TimingMode::MultipleRegion,
            3 => TimingMode::Dendy,
            _ => panic!("Error parsing timing mode")
        };

    NES2Header {
        prg_rom_size: (bytes[4] as u16) | (((bytes[9] as u16) & 0x0F) << 8),
        chr_rom_size: (bytes[5] as u16) | (((bytes[9] as u16) & 0xF0) << 4),
        nametable_arrangement: if bit_to_bool(bytes[6], 0) { NametableArrangement::Horizontal} else { NametableArrangement::Vertical },
        alt_nametable_layout: bit_to_bool(bytes[6], 3),
        trainer: bit_to_bool(bytes[6], 2),
        nvm_present: bit_to_bool(bytes[6], 1),
        mapper,
        submapper: (bytes[8] & 0xF0) >> 4,
        console_type,
        prg_ram_size: parse_shift_format(bytes[10] & 0x0F),
        prg_nvram_size: parse_shift_format((bytes[10] & 0xF0) >> 4),
        chr_ram_size: parse_shift_format(bytes[11] & 0x0F),
        chr_nvram_size: parse_shift_format((bytes[10] & 0xF0) >> 4),
        timing_mode,
        default_expansion_device: bytes[15] & 0x7F,
        misc_roms: bytes[14] & 0x03,
    }
}

// Debug method to dump contents of the raw .nes binary
pub fn dump_bytes(bytes: &[u8]) {
    for (i, byte) in bytes.iter().enumerate() {
        print!("{:x} ", byte);
        if i > 0 && i % 8 == 0 { println!(); } 
    } 
    println!();
}
