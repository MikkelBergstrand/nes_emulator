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

    if &bytes[0..4] != b"NES\x1A" { 
        return Err(RomError::UnrecognizedFormat);
    }
    let nes_header = parse_nes_header(&bytes);
    dbg!(&nes_header);
    
    let mut offset: usize = 16; // Length of header

    // ROM Size is in units of 16kB
    let prg_rom_size: usize = (nes_header.prg_rom_size as usize) << 14;
    let prg_rom_data = &bytes.get(offset..(offset+prg_rom_size)).ok_or(RomError::UnrecognizedFormat)?;
    offset += prg_rom_size;

    // Units of 8kB
    let chr_rom_size = (nes_header.chr_rom_size as usize) << 13;
    let chr_rom_data = bytes.get(offset..(offset+chr_rom_size)).ok_or(RomError::UnrecognizedFormat)?;
    // offset += chr_rom_size;

    Ok(NESData {
        header: nes_header,
        prg_rom: prg_rom_data.to_vec(),
        chr_rom: chr_rom_data.to_vec(),
    })
}

fn parse_nes_header(bytes: &[u8]) -> NES2Header {
    fn bit_to_bool(byte: u8, bit: u8) -> bool { (byte & (1 << bit)) != 0 }
    fn parse_shift_format(shift_count: u8) -> u16 {
        if shift_count == 0 {
            return 0;
        }
        return 64 << shift_count;
    }

    // NES2 or iNES?
    let is_nes2 = ((bytes[7] >> 2) & 0x02) == 2;

    if is_nes2 {
        println!("Recognized NES 2.0 format");
    } else {
        println!("Recognized iNES format");
    }


    // Following definitions are common for iNES and NES2.0
    let prg_rom_size = (bytes[4] as u16) | (((bytes[9] as u16) & 0x0F) << 8);
    let chr_rom_size =  (bytes[5] as u16) | (((bytes[9] as u16) & 0xF0) << 4);
    let nametable_arrangement = if bit_to_bool(bytes[6], 0) { NametableArrangement::Horizontal} else { NametableArrangement::Vertical };
    let nvm_present = bit_to_bool(bytes[6], 1);
    let trainer = bit_to_bool(bytes[6], 2);
    let alt_nametable_layout = bit_to_bool(bytes[6], 3);
    
    // NES2 supports 12-bit mapper number, whereas iNES supports only 8-bit
    let mapper: u16 = if is_nes2 { ((bytes[6] as u16) >> 4) 
        | ((bytes[7] as u16) & 0xF0)
        | (((bytes[8] as u16) & 0x0F) << 4)
    } else {
        ((bytes[6] as u16) >> 4) | ((bytes[7] as u16) & 0xF0)
    };
    
    let console_type = if is_nes2 {
        match bytes[7] & 0x03 {
            0 => ConsoleType::NES,
            1 => ConsoleType::VsSystem,
            2 => ConsoleType::Playchoice10,
            3 => ConsoleType::Extended,
            _ => panic!("Error parsing console type")
        }
    } else {
        match bytes[7] & 0x03 {
            0 => ConsoleType::NES,
            1 => ConsoleType::VsSystem,
            2 => ConsoleType::Playchoice10,
            _ => panic!("Error parsing console type")
        }
    };

    let timing_mode = 
        match bytes[12] & 0x03 {
            0 => TimingMode::NTSC,
            1 => TimingMode::PAL,
            2 => TimingMode::MultipleRegion,
            3 => TimingMode::Dendy,
            _ => panic!("Error parsing timing mode")
        };
    

    let prg_ram_size = if is_nes2 {
        parse_shift_format(bytes[10] & 0x0F)
    } else {
        bytes[8] as u16
    };

    // Note that bytes 9 and 10 in iNES are not supported.


    NES2Header {
        alt_nametable_layout,
        chr_rom_size,
        nametable_arrangement,
        nvm_present,
        prg_rom_size,
        trainer,
        mapper,
        submapper: (bytes[8] & 0xF0) >> 4,
        console_type,
        prg_ram_size,
        prg_nvram_size: if is_nes2 { parse_shift_format((bytes[10] & 0xF0) >> 4) } else { 0 },
        chr_ram_size: if is_nes2 { parse_shift_format(bytes[11] & 0x0F) } else { 0 },
        chr_nvram_size: if is_nes2 { parse_shift_format((bytes[10] & 0xF0) >> 4) } else { 0 },
        timing_mode: if is_nes2 { timing_mode } else { TimingMode::NTSC },
        default_expansion_device: if is_nes2 { bytes[15] & 0x7F } else { 0 },
        misc_roms: if is_nes2 { bytes[14] & 0x03 } else { 0 },
    }
}
