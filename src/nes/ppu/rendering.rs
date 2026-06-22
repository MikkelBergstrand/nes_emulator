use crate::nes::{mappers::Mapper};

use super::{PPU, flags::{PPUCTRL, PPUMask, PPUStatus}, oam::TempSpriteInfo, sprite_buffer_memory::BufferSprite};


pub const IMG_WIDTH: usize = 256;
pub const IMG_HEIGHT: usize = 240;
pub const BYTES_PER_PIXEL: usize = 4;
pub const IMG_SIZE: usize = IMG_HEIGHT*IMG_WIDTH*BYTES_PER_PIXEL;


impl PPU {
    // X-increment. PPU does this after every tile render, including
    // on the pre-render scanline
    fn horizontal_increment(&mut self) {
        if (self.v & 0x001F) == 31  {
            self.v &= !0x001F;
            self.v ^= 0x0400;
        } else {
            self.v += 1;
        }

        self.v &= 0x7FFF;

    }

    // Y-increment. PPU does this after every rendered scanline, 
    // including the pre-render scanline.
    fn vertical_increment(&mut self) {
        if (self.v & 0x7000) != 0x7000 {
            self.v += 0x1000;
        } else {
            self.v &= !0x7000;
            let mut y = (self.v & 0x03E0) >> 5;
            if y == 29 { 
                y = 0;
                self.v ^= 0x0800;
            } else if y == 31 {
                y = 0;
            } else {
                y += 1;
            }
            self.v = (self.v & !0x03E0) | (y << 5);
        }

        self.v &= 0x7FFF;
    }

    fn load_background_shifters(&mut self, mapper: &mut Box<dyn Mapper>) {
        let tile_address = 0x2000 | (self.v & 0x0FFF);
        let attribute_address = 0x23C0 | (self.v & 0x0C00)
            | ((self.v >> 4) & 0x38)
            | ((self.v >> 2) & 0x07);

        let fine_y = (self.v >> 12) & 0x07;

        // 0x1000 or 0x0000, depending on CTRL flag
        let pattern_base: u16 = (self.ctrl.contains(PPUCTRL::BG_PATTERN_ADDR) as u16) << 12;

        let tile =  self.addressor.read(mapper, tile_address) as u16;

        let mut attribute = self.addressor.read(mapper, attribute_address);
        if (self.v >> 5) & 0x02 != 0 { attribute >>= 4; } 
        if  self.v       & 0x02 != 0 { attribute >>= 2; }
        let attribute = attribute & 0x03;

        let pattern_low_addr  = pattern_base | (tile << 4) | 0 | fine_y; 
        let pattern_high_addr = pattern_low_addr | 8; 

        self.pattern_data_lb = (self.pattern_data_lb & 0xFF00) | ((self.addressor.read(mapper, pattern_low_addr) as u16));
        self.pattern_data_hb = (self.pattern_data_hb & 0xFF00) | ((self.addressor.read(mapper, pattern_high_addr) as u16));

        let lo = if attribute & 0b01 != 0 { 0xFF } else { 0x00 };
        let hi = if attribute & 0b10 != 0 { 0xFF } else { 0x00 };

        self.attribute_data_lb = (self.attribute_data_lb & 0xFF00) | lo;
        self.attribute_data_hb = (self.attribute_data_hb & 0xFF00) | hi;
    }


    // Shift data in shift registers to get the next background pixel.
    fn update_background_shifters(&mut self)  {
        self.pattern_data_lb <<= 1;
        self.pattern_data_hb <<= 1;
        self.attribute_data_lb <<= 1;
        self.attribute_data_hb <<= 1;
    }

    // Shift relevant sprite register to get the next sprite pixel.
    fn update_sprite_buffers(&mut self) {
        for i in 0..8 {
            if self.sprite_buffer_data[i].x == 0 {
                // Sprite x aligned with current pixel, or beyond
                // When beyond, the shifters only emit zeros.
                self.sprite_buffer_data[i].pattern_lo <<= 1;
                self.sprite_buffer_data[i].pattern_hi <<= 1;
            } else {
                // Wait for x-coordinate to align with 
                // current pixel
                self.sprite_buffer_data[i].x -= 1;
            }
        }
    }

    // Take data from secondary OAM, which contains the sprite data
    // relevant for the next scanline, and store it in 
    // temporary sprite buffers. Effectively is just a data 
    // transformation.
    fn load_sprite_data(&mut self, mapper: &mut Box<dyn Mapper>) {
        for i in 0..8 {
            let sprite = self.oam.temp_sprite_info[i];

            let flip_x = (sprite.attributes & 0x40) != 0;

            let pallette_idx = sprite.attributes & 0x03;
            let priority = (sprite.attributes & 0x20) != 0;


            //Assume 8x8 sprite, TODO implement 8x16 logic.
            // bit 3 of PPUCTRL index pattern table base
            // y_idx: row offset into pattern table
            let pattern_base = (self.ctrl.contains(PPUCTRL::SPRITE_PATTERN_ADDR) as u16) << 12;
            let pattern_address_lo = pattern_base
                | ((sprite.tile_index as u16) << 4) 
                | (sprite.y_pos as u16);
            let pattern_address_hi = pattern_address_lo | 8;
            
            let mut pattern_lo = self.addressor.read(mapper, pattern_address_lo);
            let mut pattern_hi = self.addressor.read(mapper, pattern_address_hi);

            if flip_x {
                pattern_lo = pattern_lo.reverse_bits();
                pattern_hi = pattern_hi.reverse_bits();
            }

            self.sprite_buffer_data[i] = BufferSprite{
                x:  self.oam.temp_sprite_info[i].x_pos,
                pattern_lo: pattern_lo, 
                pattern_hi: pattern_hi, 
                pallette: pallette_idx,
                priority: priority as u8,
                is_sprite_0: sprite.is_sprite_0,
            };

        }
    }

    // Draw tile at (xpos, ypos)
    fn draw_tile(&mut self, mapper: &mut Box<dyn Mapper>, xpos: usize, ypos: usize) {
        let bit_mux = 0x8000u16 >> self.x;

        let bg_pixel = if self.mask.contains(PPUMask::BACKGROUND_RENDER_EN) {
            let p0 = ((self.pattern_data_lb & bit_mux) != 0) as u8;
            let p1 = ((self.pattern_data_hb & bit_mux) != 0) as u8;
            p1 << 1 | p0
        } else { 
            0
        };

        let a0 = ((self.attribute_data_lb & bit_mux) != 0) as u8;
        let a1 = ((self.attribute_data_hb & bit_mux) != 0) as u8;
        let attr = (a1 << 1) | a0;

        let mut sprite_pix = 0;
        let mut sprite_priority = false;
        let mut sprite_pallette = 0;

        if self.mask.contains(PPUMask::SPRITE_RENDER_EN) {
            for i in 0..8 {
                if self.sprite_buffer_data[i].x != 0 { continue; }

                // First visible sprite in buffer memory gets priority
                if sprite_pix == 0 {
                    let s1 = (self.sprite_buffer_data[i].pattern_hi & 0x80 != 0) as u8;
                    let s0 = (self.sprite_buffer_data[i].pattern_lo & 0x80 != 0) as u8;
                    let s = (s1 << 1) | s0;

                    sprite_pix = s;
                    sprite_priority = self.sprite_buffer_data[i].priority != 0;
                    sprite_pallette = self.sprite_buffer_data[i].pallette;
                }

                // Set the sprite 0 hit flag if this sprite is sprite 0,
                // and both the background and sprite pixel is opaque (non-zero)
                if self.sprite_buffer_data[i].is_sprite_0 && bg_pixel != 0 && sprite_pix != 0 {
                    self.status.set(PPUStatus::SPRITE0_HIT, true);
                }
            }
        }

        // MUX - determine if picking sprite, background, or backdrop (neither)
        let pallette_addr = match (bg_pixel, sprite_pix, sprite_priority) {
            (0, 0, _) =>             0x3F00,
            (0, 1..=3, _) =>         0x3F10 | ((sprite_pallette as u16) << 2) | (sprite_pix as u16),
            (1..=3, 0, _) =>         0x3F00 | ((attr as u16) << 2) | (bg_pixel as u16),
            (1..=3, 1..=3, true) =>  0x3F00 | ((attr as u16) << 2) | (bg_pixel as u16),
            (1..=3, 1..=3, false) => 0x3F10 | ((sprite_pallette as u16) << 2) | (sprite_pix as u16),
            _ => panic!("Bad MUX input"),
        };

        let mut color_index = (self.addressor.read(mapper, pallette_addr)) as usize;

        if self.mask.contains(PPUMask::GREYSCALE) {
            color_index = color_index & 0x30;
        }
        let rgb = self.color_data[color_index];
        
        let output_pixel = (ypos as usize)*IMG_WIDTH + (xpos as usize);

        self.image_out[BYTES_PER_PIXEL*output_pixel+0] = rgb[0];
        self.image_out[BYTES_PER_PIXEL*output_pixel+1] = rgb[1];
        self.image_out[BYTES_PER_PIXEL*output_pixel+2] = rgb[2];
    }

    pub fn tick(&mut self, mapper: &mut Box<dyn Mapper>) {
        if self.mask.contains(PPUMask::SPRITE_RENDER_EN) || self.mask.contains(PPUMask::BACKGROUND_RENDER_EN) {
            if self.scanline == 261 && (280..=304).contains(&self.cycle){
                // vert(v) = vert(t);
                self.v = (self.v & !0x7BE0) | (self.t & 0x7BE0);
            }

            if (0..=239).contains(&self.scanline) || self.scanline == 261 {
                if self.cycle == 0 {
                    self.oam.clear_secondary_oam();
                }

                if (1..=256).contains(&self.cycle) || (321..=336).contains(&self.cycle) {
                    if self.cycle <= 256 && self.scanline < 240 {
                        self.draw_tile(mapper, self.cycle-1, self.scanline);
                    }

                    if(1..=256).contains(&self.cycle) {
                        self.update_sprite_buffers();
                    }

                    self.update_background_shifters();

                    if self.cycle % 8 == 0 { 
                        self.load_background_shifters(mapper); 
                        self.horizontal_increment();
                    }
                }
                
                if self.cycle == 256 {
                    self.vertical_increment();
                }

                if self.cycle == 257 { 
                    //horiz(v) = horiz(t);
                    self.v = (self.v & !0x041F) | (self.t & 0x041F); 
                }

                //OAMADDR behavior
                if (257..=320).contains(&self.cycle) {
                    self.oam_addr = 0;
                }
            }

            if (0..=239).contains(&self.scanline) && self.cycle == 256 {
                self.evaluate_sprites(self.scanline+1);
                self.load_sprite_data(mapper);
            }
        } else if (1..=256).contains(&self.cycle) && (0..240).contains(&self.scanline) {
            // We must still render nothing even when rendering is disabled
            // to wipe out the screen.
            self.draw_tile(mapper, self.cycle-1, self.scanline);

        }


        if self.scanline == 241 && self.cycle == 1 {
            // Set vblank flag
            self.status.set(PPUStatus::VBLANK, true);
            // Must have NMI enabled in PPUCTRL
            if self.ctrl.contains(PPUCTRL::VBLANK_NMI_EN) {
                // Only trigger NMI on rising edge of STATUS bit.
                if !self.nmi_lineout {
                    self.pending_nmi = true;
                }
                self.nmi_lineout = true;
            }
            self.image_ready = true;
        }

        if self.scanline == 261 && self.cycle == 1 {
            self.status.remove(PPUStatus::VBLANK);
            self.status.remove(PPUStatus::SPRITE0_HIT);
            self.status.remove(PPUStatus::SPRITE_OVERFLOW);
            self.image_ready = false;

        }

        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
            }
        }
    }

    pub fn evaluate_sprites(&mut self, scanline: usize) {
        self.oam.clear_secondary_oam();
        
        let mut found = 0;  
        let mut n = 0;
        let mut m = 0;

        for _ in 0..64 {
            let y_pos = self.oam.sprites[4*n];
            // 9-bit difference of scanline and sprite y_pos
            let cmp = (scanline as u16).wrapping_sub((y_pos.wrapping_add(1)) as u16) as u8;

            // Check if scanline intersects sprite
            if cmp < 8 {
                let flip_y = (self.oam.sprites[4*n+2] & 0x80) != 0;
                self.oam.temp_sprite_info[found] = TempSpriteInfo {
                    y_pos: if flip_y { 7 - cmp } else { cmp }, // Flip y_pos bits if flip
                    tile_index: self.oam.sprites[4*n+1],
                    attributes: self.oam.sprites[4*n+2],
                    x_pos:      self.oam.sprites[4*n+3],
                    is_sprite_0: n == 0,
                };
                //println!("Sprite at x={} y={}", self.oam.temp_sprite_info[found].x_pos, self.oam.temp_sprite_info[found].y_pos);
                found += 1;
            }
            n += 1;
            if found >= 8 {
                break;
            }
        }

        // Step 3: Now prite memory is full.
        // This step is intended to set the sprite overflow flag.
        // The routine is buggy and does not work as intended.
        while n < 64 {
            let y_pos = self.oam.sprites[4*n+m];

            let cmp = (self.scanline as u16).wrapping_sub((y_pos) as u16);
            if cmp < 8 {
                self.status.insert(PPUStatus::SPRITE_OVERFLOW);
                m += 3;
                if m > 3 {
                    n += 1;
                    m = m % 3;
                } 

            } else {
                n += 1; m += 1; // Bug here: m should not be incremented.
                if m > 3 {
                    n += 1;
                    m = m % 3;
                } 
                if n >= 64 {
                    break;
                }
            }
        }
    }
}
