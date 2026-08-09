// Set lower 8 bits of an APU channel timer.
// Maintains the upper bits as they were.
macro_rules! timer_low {
    ($period:expr, $data:expr) => {
        ($period & 0xFF00) | ($data as u16)
    };
}

// Sets upper 3 bits of an APU channel timer.
// Maintains the lower bits as they were.
macro_rules! timer_high {
    ($period:expr, $data:expr) => {
        ((($data & 0x07) as u16) << 8) | ($period & 0x00FF)
    };
}

pub(crate) use {timer_high, timer_low};
