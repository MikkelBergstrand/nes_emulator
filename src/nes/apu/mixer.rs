use crate::nes::apu::APUChannels;

pub trait APUMixer {
    fn mix(&self, channels: &APUChannels) -> f32;
    fn new() -> Self;
}

pub struct APULookupTableMixer {
    pulse_lookup: [f32; 31],
}

impl APUMixer for APULookupTableMixer {
    fn new() -> Self {
        let mut pulse_lookup = [0f32; 31];
        for i in 0..pulse_lookup.len() {
            pulse_lookup[i] = 95.52 / (8128.0 / i as f32 + 100.0);
        }

        Self { pulse_lookup }
    }

    fn mix(&self, channels: &APUChannels) -> f32 {
        let pulse_sum = channels.pulse_1.get_output() + channels.pulse_2.get_output();
        self.pulse_lookup[pulse_sum as usize]
    }
}
