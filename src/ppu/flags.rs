
#[derive(Debug, Clone, Copy)]
pub struct PPUReg;

impl PPUReg {
    pub const CTRL: u8    = 0;
    pub const MASK: u8   = 1;
    pub const STATUS: u8  = 2;
    pub const OAMADDR: u8  = 3;
    pub const OAMDATA: u8 = 4;
    pub const SCROLL: u8   = 5;
    pub const ADDR: u8     = 6;
    pub const DATA: u8    = 7;
}

enum PPURegister {

}
