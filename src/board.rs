#![allow(dead_code)]

use crate::utils::{PlayerSlotValue, SlotCoord, SlotPosition, SlotUpdateStatus, SlotValue};
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

    /// update the specified slot on the board using the value provided. Usually invoked by a Player in order to put his/her piece on a specific slot on the board
    pub fn update(
        &mut self,
        slot_position: SlotPosition,
        player_slot_value: &PlayerSlotValue,
    ) -> SlotUpdateStatus {
        let coords = SlotCoord::from(slot_position);
        // check if slot is already played by a player
        let slot = &self.state[coords.0][coords.1];

        if let SlotValue::PlayerX | SlotValue::PlayerO = slot {
            return SlotUpdateStatus::Duplicate;
        }

        self.state[coords.0][coords.1] = (player_slot_value.clone()).try_into().unwrap();
        return SlotUpdateStatus::Success;
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
