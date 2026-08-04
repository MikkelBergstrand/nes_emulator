use std::usize;

// Also referred to as "Secondary OAM" in NesDev
#[derive(Copy, Clone, Debug)]
pub struct TempSpriteInfo {
    pub y_pos: u8,
    pub tile_index: u8,
    pub attributes: u8,
    pub x_pos: u8,
    pub is_sprite_0: bool,
}

impl TempSpriteInfo {
    // Secondary RAM is cleared by setting all the values to 0xFF
    pub fn blank() -> Self {
        TempSpriteInfo {
            y_pos: 0xFF,
            tile_index: 0xFF,
            attributes: 0xFF,
            x_pos: 0xFF,
            is_sprite_0: false,
        }
    }
}

pub struct OAM {
    // Sprite data, usually set through OAMDMA.
    // Room for 64 sprites, 4 bytes per sprite.
    // Index 0 = y_pos
    // Index 1 = tile_index
    // Index 2 = attributes
    // Index 3 = x_pos
    pub sprites: [u8; 64 * 4],

    // Secondary OAM storage. Room for 8 sprites per scanline
    pub temp_sprite_info: [TempSpriteInfo; 8],
}

impl OAM {
    pub fn new() -> Self {
        OAM {
            sprites: [0u8; 64 * 4],
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

    // Evaulate sprites to be shown on the inputted scanline.
    // Requires sprite height (either 8 or 16) determined by PPUCTRL bit 5
    // Returns number of visible sprites + whether sprite overflow flag should be set
    pub fn evaluate_sprites(&mut self, scanline: usize, sprite_height: u8) -> (u8, bool) {
        let mut n = 0;
        let mut found = 0;

        while n < 256 {
            let y_pos = self.sprites[n];
            // 9-bit difference of scanline and sprite y_pos
            let cmp = (scanline as i16) - (y_pos as i16);

            // Check if scanline intersects sprite
            if cmp >= 0 && cmp < sprite_height.into() {
                let cmp = cmp as u8;
                let flip_y = (self.sprites[n + 2] & 0x80) != 0;
                self.temp_sprite_info[found] = TempSpriteInfo {
                    y_pos: if flip_y { sprite_height - 1 - cmp } else { cmp },
                    tile_index: self.sprites[n + 1],
                    attributes: self.sprites[n + 2],
                    x_pos: self.sprites[n + 3],
                    is_sprite_0: n == 0,
                };
                found += 1;
            }
            n += 4;
            if found >= 8 {
                break;
            }
        }

        // Step 3: Now sprite memory is full.
        // This step is intended to set the sprite overflow flag.
        // The routine on the NES is buggy and does not work as intended.
        while n < 256 {
            let y_pos = self.sprites[n];

            let cmp = (scanline as i16) - (y_pos as i16);
            if cmp >= 0 && cmp < sprite_height.into() {
                return (found as u8, true);
            } else {
                n += 5;
            }
        }

        (found as u8, false)
    }
}
