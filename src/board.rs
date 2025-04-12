#![allow(dead_code)]

use crate::utils::{
    PlayerSlotValue, SlotCoord, SlotPosition, SlotUpdateStatus, SlotValue, WinState,
};
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

    pub fn check_match(&self) -> WinState {
        // checking the rows
        //  ar, as, at
        if self.state[0][0] == self.state[0][1] && self.state[0][1] == self.state[0][2] {
            return WinState::from(self.state[0][0].clone());
        }

        //  br, bs, bt
        if self.state[1][0] == self.state[1][1] && self.state[1][1] == self.state[1][2] {
            return WinState::from(self.state[1][0].clone());
        }

        //  cr, cs, ct
        if self.state[2][0] == self.state[2][1] && self.state[2][1] == self.state[2][2] {
            return WinState::from(self.state[2][0].clone());
        }

        // checking the columns
        //  ar, br, cr
        if self.state[0][0] == self.state[1][0] && self.state[1][0] == self.state[2][0] {
            return WinState::from(self.state[0][0].clone());
        }

        //  as, bs, cs
        if self.state[0][1] == self.state[1][1] && self.state[1][1] == self.state[2][1] {
            return WinState::from(self.state[0][1].clone());
        }

        //  at, bt, ct
        if self.state[0][2] == self.state[1][2] && self.state[1][2] == self.state[2][2] {
            return WinState::from(self.state[0][2].clone());
        }

        // check the diagonals
        // ar, bs, ct
        if self.state[0][0] == self.state[1][1] && self.state[1][1] == self.state[2][2] {
            return WinState::from(self.state[0][0].clone());
        }

        // at, bs, cr
        if self.state[0][2] == self.state[1][1] && self.state[1][1] == self.state[2][0] {
            return WinState::from(self.state[0][2].clone());
        }

        // if slots are all filled, then it's a draw
        let is_full = {
            let mut is_full = true;
            for i in self.state.concat() {
                if i == SlotValue::Empty {
                    is_full = false;
                    break;
                }
            }
            is_full
        };

        if is_full {
            return WinState::Draw;
        }

        return WinState::None;
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
