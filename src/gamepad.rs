use gilrs::{Button, Gilrs};

use crate::inputs::InputFlag;

pub struct Gamepad {
    gilrs: gilrs::Gilrs,
}

impl Gamepad {
    pub fn new() -> Self {
        Self {
            gilrs: Gilrs::new().unwrap(),
        }
    }

    /// Returns the currently held buttons on the first connected pad. Call
    /// once per frame.
    pub fn poll(&mut self) -> InputFlag {
        // gilrs only emits an event when a button changes, but draining the
        // queue is what refreshes its cached per-pad state. So pump the queue
        // for that side effect and then read the state — deriving the buttons
        // from the events themselves makes a held button read as released on
        // every poll after the one that saw the press.
        while self.gilrs.next_event().is_some() {}

        let mut inputs = InputFlag::empty();
        let Some((_, pad)) = self.gilrs.gamepads().next() else {
            return inputs;
        };

        inputs.set(InputFlag::A, pad.is_pressed(Button::South));
        inputs.set(InputFlag::B, pad.is_pressed(Button::West));
        inputs.set(InputFlag::START, pad.is_pressed(Button::Start));
        inputs.set(InputFlag::SELECT, pad.is_pressed(Button::Select));

        inputs.set(InputFlag::UP, pad.is_pressed(Button::DPadUp));
        inputs.set(InputFlag::DOWN, pad.is_pressed(Button::DPadDown));
        inputs.set(InputFlag::LEFT, pad.is_pressed(Button::DPadLeft));
        inputs.set(InputFlag::RIGHT, pad.is_pressed(Button::DPadRight));

        inputs
    }
}
