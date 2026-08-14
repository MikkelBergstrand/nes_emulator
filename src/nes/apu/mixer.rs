use crate::nes::apu::APUChannels;

pub trait APUMixer {
    fn mix(&self, channels: &APUChannels) -> f32;
    fn new() -> Self;
}

pub struct APULookupTableMixer {
    pulse_lookup: [f32; 31],
    tnd_lookup: [f32; 203],
}

impl APUMixer for APULookupTableMixer {
    fn new() -> Self {
        let mut pulse_lookup = [0f32; 31];
        let mut tnd_lookup = [0f32; 203];
        for i in 0..pulse_lookup.len() {
            pulse_lookup[i] = 95.52 / (8128.0 / i as f32 + 100.0);
        }

        for i in 0..tnd_lookup.len() {
            tnd_lookup[i] = 163.67 / (24329.0 / i as f32 + 100.0);
        }

        Self {
            pulse_lookup,
            tnd_lookup,
        }
    }

    fn mix(&self, channels: &APUChannels) -> f32 {
        let pulse_sum = channels.pulse_1.get_output() + channels.pulse_2.get_output();
        let tnd_sum = 3 * channels.triangle.get_output() + 2 * channels.noise.get_output();
        self.pulse_lookup[pulse_sum as usize] + self.tnd_lookup[tnd_sum as usize]
    }
}
