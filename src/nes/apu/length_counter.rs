#[rustfmt::skip]
pub const LENGTH_COUNTER_TABLE: [u8; 0x20] = [
    // even index: note length            odd index: linear ramp
    10,  254, // 0x00 sixteenth           0x01
    20,  2,   // 0x02 eighth              0x03
    40,  4,   // 0x04 quarter             0x05
    80,  6,   // 0x06 half                0x07
    160, 8,   // 0x08 whole               0x09
    60,  10,  // 0x0A dotted quarter      0x0B
    14,  12,  // 0x0C eighth triplet      0x0D
    26,  14,  // 0x0E quarter triplet     0x0F
    12,  16,  // 0x10 sixteenth           0x11
    24,  18,  // 0x12 eighth              0x13
    48,  20,  // 0x14 quarter             0x15
    96,  22,  // 0x16 half                0x17
    192, 24,  // 0x18 whole               0x19
    72,  26,  // 0x1A dotted quarter      0x1B
    16,  28,  // 0x1C eighth triplet      0x1D
    32,  30,  // 0x1E quarter triplet     0x1F
];

pub struct LengthCounter {
    length_counter: u8,
    enabled: bool,
    halt: bool,
}

impl LengthCounter {
    pub fn new() -> Self {
        Self {
            length_counter: 0,
            halt: false,
            enabled: false,
        }
    }

    pub fn set_halt(&mut self, v: bool) {
        self.halt = v;
    }

    pub fn set_counter(&mut self, v: u8) {
        if !self.enabled {
            return;
        }

        self.length_counter = LENGTH_COUNTER_TABLE[v as usize];
    }

    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = v;
        if !v {
            self.length_counter = 0;
        }
    }

    pub fn tick(&mut self) {
        if self.length_counter > 0 && !self.halt && self.enabled {
            self.length_counter -= 1;
        }
    }

    pub fn active(&self) -> bool {
        return self.length_counter > 0;
    }
}
