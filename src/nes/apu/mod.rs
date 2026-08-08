use crate::nes::{F_CPU, apu::channel::PulseChannel};

mod adressing;
pub mod channel;

pub const F_APU: f32 = F_CPU / 2.0;

pub struct APU {
    pub pulse_channel_1: PulseChannel,
    pub pulse_channel_2: PulseChannel,
    status: u8,
    frame_counter: u8,
}

impl APU {
    pub fn new() -> Self {
        APU {
            pulse_channel_1: PulseChannel::new(),
            pulse_channel_2: PulseChannel::new(),
            status: 0,
            frame_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        self.pulse_channel_1.tick(self.status & 1 != 0);
    }

    pub fn get_output(&self) -> f32 {
        self.pulse_channel_1.get_output()
    }
}

pub struct APUStatus {}
