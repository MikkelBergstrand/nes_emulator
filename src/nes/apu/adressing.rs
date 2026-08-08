use crate::nes::apu::APU;

impl APU {
    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x4000..=0x4003 => {
                self.pulse_channel_1.set((addr - 0x4000) as u8, data);
            }
            0x4015 => {
                self.status = data;
            }
            _ => (),
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            _ => 0,
        }
    }
}
