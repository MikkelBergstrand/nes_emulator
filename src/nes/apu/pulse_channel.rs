use crate::nes::apu::{
    channel::{timer_high, timer_low},
    divider::Divider,
    envelope::Envelope,
    length_counter::LengthCounter,
    sweeper::Sweeper,
};

// Defines the output of the sequencer
// First index is by duty cycle mode, which may be set by writing to 0x4000 or 0x4004
// Second index is the current sequencer step.
// The sequencer is clocked every time the timer value (t) wraps around to zero, see tick()
const PULSE_SEQUENCER: &[&[u8]] = &[
    &[0, 1, 0, 0, 0, 0, 0, 0], //12.5%
    &[0, 1, 1, 0, 0, 0, 0, 0], //25%
    &[0, 1, 1, 1, 1, 0, 0, 0], //50%
    &[1, 0, 0, 1, 1, 1, 1, 1], //25% negated
];

pub struct PulseChannel {
    duty: u8,

    enabled: bool,

    sequencer_step: u8,

    divider: Divider,
    envelope: Envelope,
    sweeper: Sweeper,
    length_counter: LengthCounter,
}

impl PulseChannel {
    pub fn new() -> Self {
        PulseChannel {
            enabled: false,
            duty: 0,
            sequencer_step: 0,
            divider: Divider::new(0),
            envelope: Envelope::new(),
            sweeper: Sweeper::new(),
            length_counter: LengthCounter::new(),
        }
    }

    pub fn set(&mut self, reg: u8, data: u8) {
        match reg {
            0 => {
                self.duty = (data & 0xC0) >> 6;
                self.envelope
                    .set_params((data & 0x10) != 0, data & 0xF, data & 0x20 != 0);
                self.length_counter.set_halt(data & 0x20 != 0);
            }
            1 => {
                self.sweeper.enabled = (data & 0x80) != 0;
                self.sweeper.period = (data & 0x70) >> 4;
                self.sweeper.negate = (data & 0x08) != 0;
                self.sweeper.shift = data & 0x07;
                self.sweeper.reset();
            }
            2 => self
                .divider
                .set_period(timer_low!(self.divider.get_period(), data)),
            3 => {
                self.divider
                    .set_period(timer_high!(self.divider.get_period(), data));
                self.length_counter.set_counter((data & 0xF8) >> 3);
                self.envelope.set_start();

                // Side effect of write
                self.sequencer_step = 0;
            }
            _ => panic!("PulseChannel invalid address"),
        }
    }

    pub fn tick(&mut self) {
        // Clock the sequencer on the transition from 0 to period
        if self.divider.tick() {
            self.sequencer_step = (self.sequencer_step + 1) % 8;
        }
    }

    pub fn tick_envelope(&mut self) {
        self.envelope.tick();
    }

    pub fn tick_sweep(&mut self) {
        self.divider
            .set_period(self.sweeper.tick(self.divider.get_period()));
    }

    pub fn tick_length_counter(&mut self) {
        self.length_counter.tick();
    }

    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = v;
        self.length_counter.set_enabled(v);
    }

    fn enabled(&self) -> bool {
        self.length_counter.active() && !self.sweeper.mute(self.divider.get_period())
    }

    pub fn get_output(&self) -> u16 {
        if self.enabled() {
            // PULSE_SEQUENCER contains the  current duty mode (0-3) and the
            // current sequencer step, which is an 8-step looping sequence
            // volume is determined by the envelope
            (self.envelope.volume()
                * PULSE_SEQUENCER[self.duty as usize][self.sequencer_step as usize])
                as u16
        } else {
            0
        }
    }
}
