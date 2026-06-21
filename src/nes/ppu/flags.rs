use bitflags::bitflags;

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

bitflags! {
    pub struct PPUCTRL: u8 {
        const BASE_NAMETABLE_ADDR = 3;
        const VRAM_INCREMENT = 1 << 2;
        const SPRITE_PATTERN_ADDR = 1 << 3;
        const BG_PATTERN_ADDR = 1 << 4;
        const SPRITE_SIZE = 1 << 5;
        const MASTER_SLAVE_SEL = 1 << 6;
        const VBLANK_NMI_EN = 1 << 7;
    }
}

bitflags! {
    pub struct PPUMask: u8 {
        const GREYSCALE = 1 << 0;
        const SHOW_BG_LEFTMOST = 1 << 1;
        const SHOW_SPRITES_LEFTMOST = 1 << 2;
        const BACKGROUND_RENDER_EN = 1 << 3;
        const SPRITE_RENDER_EN = 1 << 4;
        const EMPH_RED = 1 << 5;
        const EMPH_GREEN = 1 << 6;
        const EMPH_BLUE = 1 << 7;
    }
}

bitflags! {
    pub struct PPUStatus: u8 {
        const SPRITE_OVERFLOW = 1 << 5;
        const SPRITE0_HIT = 1 << 6;
        const VBLANK = 1 << 7;
    }
}
