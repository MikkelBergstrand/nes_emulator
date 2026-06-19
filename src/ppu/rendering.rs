use crate::ppu::PPU;


pub const IMG_WIDTH: usize = 256;
pub const IMG_HEIGHT: usize = 240;
pub const BYTES_PER_PIXEL: usize = 4;
pub const IMG_SIZE: usize = IMG_HEIGHT*IMG_WIDTH*BYTES_PER_PIXEL;


impl PPU {
    fn inc_hori(&mut self) {
        // X-increment (when next tile is reached)
        if (self.v & 0x001F) == 31  {
            self.v &= !0x001F;
            self.v ^= 0x0400;
        } else {
            self.v += 1;
        }

        self.v &= 0x7FFF;

    }

    fn inc_vert(&mut self) {
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

    fn load_shifters(&mut self) {
        let tile_address = 0x2000 | (self.v & 0x0FFF);
        let attribute_address = 0x23C0 | (self.v & 0x0C00) 
            | ((self.v >> 4) & 0x38) 
            | ((self.v >> 2) & 0x07);

        let fine_y = (self.v >> 12) & 0x07;

        let pattern_base = self.base_background_pattern_address();
        let tile =  self.addressor.read(tile_address) as u16;

        let mut attribute = self.addressor.read(attribute_address);
        if (self.v >> 5) & 0x02 != 0 { attribute >>= 4; }
        if  self.v       & 0x02 != 0 { attribute >>= 2; }
        let attribute = attribute & 0x03;

        let pattern_low_addr  = pattern_base | (tile << 4) | 0 | fine_y; 
        let pattern_high_addr = pattern_base | (tile << 4) | 8 | fine_y; 

        self.pattern_data_lb = (self.pattern_data_lb & 0xFF00) | ((self.addressor.read(pattern_low_addr) as u16));
        self.pattern_data_hb = (self.pattern_data_hb & 0xFF00) | ((self.addressor.read(pattern_high_addr) as u16));

        let lo = if attribute & 0b01 != 0 { 0xFF } else { 0x00 };
        let hi = if attribute & 0b10 != 0 { 0xFF } else { 0x00 };

        self.attribute_data_lb = (self.attribute_data_lb & 0xFF00) | lo;
        self.attribute_data_hb = (self.attribute_data_hb & 0xFF00) | hi;
    }

    fn update_shifters(&mut self)  {
        self.pattern_data_lb <<= 1;
        self.pattern_data_hb <<= 1;
        self.attribute_data_lb <<= 1;
        self.attribute_data_hb <<= 1;
    }

    fn draw_tile(&mut self, xpos: usize, ypos: usize) {
        let bit_mux = 0x8000u16 >> self.x;
        let p0 = ((self.pattern_data_lb & bit_mux) != 0) as u8;
        let p1 = ((self.pattern_data_hb & bit_mux) != 0) as u8;
        let bg_pixel = (p1 << 1) | p0;
        

        let a0 = ((self.attribute_data_lb & bit_mux) != 0) as u8;
        let a1 = ((self.attribute_data_hb & bit_mux) != 0) as u8;
        let attr = (a1 << 1) | a0;

        let pallette_addr = if bg_pixel == 0 {
            0x3F00
        } else {
            0x3F00 | ((attr as u16) << 2) | (bg_pixel as u16)
        };
        let mut color_index = (self.addressor.read(pallette_addr) & 0x3F) as usize;

        if self.greyscale() {
            color_index = color_index & 0x30;
        }
        let rgb = self.color_data[color_index];
        
        let output_pixel = (ypos as usize)*IMG_WIDTH + (xpos as usize);
        self.image_out[BYTES_PER_PIXEL*output_pixel+0] = rgb[0];
        self.image_out[BYTES_PER_PIXEL*output_pixel+1] = rgb[1];
        self.image_out[BYTES_PER_PIXEL*output_pixel+2] = rgb[2];
    }

    pub fn tick(&mut self) {
        if !self.enable_sprites() && !self.enable_background() {
            // Only render if mask allows it
            if self.scanline == 1 && self.cycle == 1 {
                self.status |= 0x40; //DEBUG: set sprite 0 flag manually
            }

            if self.scanline == 261 && (280..=304).contains(&self.cycle){
                // vert(v) = vert(t);
                self.v = (self.v & !0x7BE0) | (self.t & 0x7BE0);
            }

            if (0..=239).contains(&self.scanline) || self.scanline == 261 {
                if (2..=257).contains(&self.cycle) || (321..=337).contains(&self.cycle) {
                    self.update_shifters();
                }

                if (2..=257).contains(&self.cycle) || (321..=337).contains(&self.cycle) {
                    if (self.cycle - 1) % 8 == 0 { self.load_shifters(); }
                    if (self.cycle - 1) % 8 == 7 { self.inc_hori(); }
                }

                if self.cycle == 256 {
                    self.inc_vert();
                }

                if self.cycle == 257 { 
                    //horiz(v) = horiz(t);
                    self.v = (self.v & !0x041F) | (self.t & 0x041F); 
                }

                if (1..=256).contains(&self.cycle) && self.scanline < 240 {
                    self.draw_tile(self.cycle-1, self.scanline);
                }

                //OAMADDR behavior
                if (257..=320).contains(&self.cycle) {
                    self.oam_addr = 0;
                }
            }
        }


        if self.scanline == 241 && self.cycle == 1 {
            // Set vblank flag
            self.status |= 0x80;
            // Must have NMI enabled in PPUCTRL
            if self.vblank_nmi_enable() {
                // Only trigger NMI on rising edge of STATUS bit.
                if !self.nmi_lineout {
                    self.pending_nmi = true;
                }
                self.nmi_lineout = true;
            }
            self.image_ready = true;
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
}
