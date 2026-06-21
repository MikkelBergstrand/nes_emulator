#[derive(Copy, Clone, Debug)]
pub struct BufferSprite {
    pub pattern_hi: u8,
    pub pattern_lo: u8,
    pub pallette: u8,
    pub priority: u8,
    pub x: u8,
    pub is_sprite_0: bool,
}

impl BufferSprite {
    pub fn new() -> Self {
        BufferSprite { pattern_hi: 0, pattern_lo: 0, pallette: 0, priority: 0, x: 0, is_sprite_0: false }
    }
}
