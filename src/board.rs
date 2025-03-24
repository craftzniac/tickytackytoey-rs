use crate::utils::{PlayerSlotValue, SlotValue};
use std::fmt;

#[derive(Debug)]
pub(crate) struct Board {
    state: [[SlotValue; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Self {
            state: [
                [SlotValue::Empty, SlotValue::Empty, SlotValue::Empty],
                [SlotValue::Empty, SlotValue::Empty, SlotValue::Empty],
                [SlotValue::Empty, SlotValue::Empty, SlotValue::Empty],
            ],
        }
    }

    /// update the board state. Usually invoked by a Player in order to play a turn on the board
    pub fn update(&mut self, slot_position: SlotRepr, slot_value: PlayerSlotValue) {

        // TODO: I need to map from some user-friendly string for representing a position on the
        // board, to actual position within the 2d array. This way, player has a way to point to a
        // specific slot on the board which he/she wants to place their piece
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"      r   s   t

a   | {} | {} | {} |
b   | {} | {} | {} |
c   | {} | {} | {} |

move syntax: <row><col> e.g ar
        "#,
            self.state[0][0],
            self.state[0][1],
            self.state[0][2],
            self.state[1][0],
            self.state[1][1],
            self.state[1][2],
            self.state[2][0],
            self.state[2][1],
            self.state[2][2],
        )
    }
}
