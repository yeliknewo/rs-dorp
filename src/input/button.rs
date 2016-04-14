use input::{ButtonState};

#[derive(Debug, Copy, Clone)]
pub struct Button {
    tick_number: u64,
    state: ButtonState,
}

impl Button {
    pub fn new(tick_number: u64, state: ButtonState) -> Button {
        Button {
            tick_number: tick_number,
            state: state,
        }
    }

    pub fn get_tick_number(&self) -> u64 {
        self.tick_number
    }

    pub fn get_state(&self) -> ButtonState {
        self.state
    }
}
