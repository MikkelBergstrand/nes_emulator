use crate::nes::apu::{APU, mixer::APUMixer};

impl<T: APUMixer> APU<T> {
    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x4000..=0x4003 => {
                self.channels.pulse_1.set((addr - 0x4000) as u8, data);
            }
            0x4004..=0x4007 => {
                self.channels.pulse_2.set((addr - 0x4004) as u8, data);
            }
            0x4015 => {
                self.status = data;
            }
            0x4017 => self.set_frame_counter(data),
            _ => (),
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            _ => 0,
        }
    }
}
