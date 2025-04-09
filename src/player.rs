#![allow(dead_code)]
use crate::utils::{PlayerSlotValue, SlotPosition, get_user_input};

#[derive(Clone)]
pub(crate) struct Player {
    name: String,
    pub slot_value: PlayerSlotValue,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.slot_value)
    }
}

impl Player {
    pub fn new(name: String, slot_value: PlayerSlotValue) -> Self {
        Self { name, slot_value }
    }

    pub fn get_move(&self) -> SlotPosition {
        // accept input from cmd
        let prompt_string = format!("{} move: ", self);
        return get_user_input(prompt_string);
    }
}
