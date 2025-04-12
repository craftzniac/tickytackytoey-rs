#![allow(dead_code)]
use std::{
    fmt::{self, Display},
    io::{self, Write},
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SlotValue {
    PlayerO,
    PlayerX,
    Empty,
}

impl fmt::Display for SlotValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlotValue::Empty => {
                write!(f, " ")
            }
            SlotValue::PlayerO => {
                write!(f, "o")
            }
            SlotValue::PlayerX => {
                write!(f, "x")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlayerSlotValue {
    PlayerO = SlotValue::PlayerO as isize,
    PlayerX = SlotValue::PlayerX as isize,
}

impl fmt::Display for PlayerSlotValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerSlotValue::PlayerO => {
                write!(f, "o")
            }
            PlayerSlotValue::PlayerX => {
                write!(f, "x")
            }
        }
    }
}

//get a PlayerSlotValue from a SlotValue
impl TryFrom<SlotValue> for PlayerSlotValue {
    type Error = String;
    fn try_from(value: SlotValue) -> Result<Self, Self::Error> {
        match value {
            SlotValue::PlayerX => Ok(PlayerSlotValue::PlayerX),
            SlotValue::PlayerO => Ok(PlayerSlotValue::PlayerO),
            SlotValue::Empty => Err("Not a recognized player slot value".into()),
        }
    }
}

impl TryFrom<PlayerSlotValue> for SlotValue {
    type Error = String;
    fn try_from(value: PlayerSlotValue) -> Result<Self, Self::Error> {
        match value {
            PlayerSlotValue::PlayerO => Ok(SlotValue::PlayerO),
            PlayerSlotValue::PlayerX => Ok(SlotValue::PlayerX),
        }
    }
}

#[derive(Debug)]
pub enum SlotPosition {
    // row 1
    AR,
    AS,
    AT,
    // row 2
    BR,
    BS,
    BT,
    // row 3
    CR,
    CS,
    CT,
}

// get a SlotPosition from a string
impl TryFrom<String> for SlotPosition {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        // why did I have to trim this???
        match value.to_lowercase().as_str().trim() {
            "ar" => Ok(SlotPosition::AR),
            "as" => Ok(SlotPosition::AS),
            "at" => Ok(SlotPosition::AT),

            "br" => Ok(SlotPosition::BR),
            "bs" => Ok(SlotPosition::BS),
            "bt" => Ok(SlotPosition::BT),

            "cr" => Ok(SlotPosition::CR),
            "cs" => Ok(SlotPosition::CS),
            "ct" => Ok(SlotPosition::CT),
            _ => Err("Invalid slot position".to_string()),
        }
    }
}

// TODO: find out why you get a `SliceIndex<[[SlotValue; 3]] not implemented for u8 but is
// implemented for usize
pub type SlotCoord = (usize, usize);

// get the coordinates of a slot from it's SlotPosition
impl From<SlotPosition> for SlotCoord {
    fn from(value: SlotPosition) -> Self {
        match value {
            SlotPosition::AR => (0, 0),
            SlotPosition::AS => (0, 1),
            SlotPosition::AT => (0, 2),

            SlotPosition::BR => (1, 0),
            SlotPosition::BS => (1, 1),
            SlotPosition::BT => (1, 2),

            SlotPosition::CR => (2, 0),
            SlotPosition::CS => (2, 1),
            SlotPosition::CT => (2, 2),
        }
    }
}

pub enum WinState {
    PlayerO,
    PlayerX,
    None,
    Draw,
}

impl From<SlotValue> for WinState {
    fn from(value: SlotValue) -> Self {
        match value {
            SlotValue::PlayerO => WinState::PlayerO,
            SlotValue::PlayerX => WinState::PlayerX,
            SlotValue::Empty => WinState::None,
        }
    }
}

impl Display for WinState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rep: String;

        match self {
            WinState::None => rep = "None".into(),
            WinState::Draw => rep = "Draw".into(),
            WinState::PlayerX => rep = "Player X".into(),
            WinState::PlayerO => rep = "Player O".into(),
        }

        write!(f, "{}", rep)
    }
}

pub fn get_user_input(prompt: String) -> SlotPosition {
    let mut user_input = String::new();
    print!("{} ", prompt);
    io::stdout().flush().unwrap(); // make sure prompt is printed before asking for input

    io::stdin().read_line(&mut user_input).unwrap();
    // try convert the user input to a Slot Position
    let res = SlotPosition::try_from(user_input);
    match res {
        Ok(slot_position) => {
            // return the slot position
            slot_position
        }
        Err(_err) => return get_user_input(prompt),
    }
}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

#[derive(PartialEq)]
pub enum SlotUpdateStatus {
    Duplicate,
    Success,
}

impl Display for SlotUpdateStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlotUpdateStatus::Duplicate => write!(f, "Slot is already occupied."),
            SlotUpdateStatus::Success => write!(f, "Slot has been update"),
        }
    }
}
