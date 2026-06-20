use std::usize;


#[derive(Debug)]
pub struct Sprite {
    y_pos: u8,
    tile_index: u8,
    attributes: u8,
    x_pos: u8,
}

// Also referre to as "Secondary OAM" in NesDev
#[derive(Copy, Clone, Debug)]
pub struct TempSpriteInfo {
    pub y_pos:  u8,
    pub tile_index: u8,
    pub attributes: u8,
    pub x_pos: u8,
    pub is_sprite_0: bool,
}

impl TempSpriteInfo {
    pub fn blank() -> Self {
        TempSpriteInfo { y_pos: 0xFF, tile_index: 0xFF, attributes: 0xFF, x_pos: 0xFF, is_sprite_0: false }
    }
}

pub struct OAM {
    pub sprites: [u8; 64*4],
    pub temp_sprite_info: [TempSpriteInfo; 8],
}

impl OAM {
    pub fn new() -> Self {
        OAM {
            sprites: [0u8; 64*4],
            temp_sprite_info: [TempSpriteInfo::blank(); 8],
        } 
    }

    pub fn from_dma(&mut self, offset: u8, values: &[u8]) {
        for i in 0..=255 {
            self.sprites[offset.wrapping_add(i) as usize] = values[i as usize];
        }
    }

    pub fn clear_secondary_oam(&mut self) {
        self.temp_sprite_info = [TempSpriteInfo::blank(); 8];
    }

}
