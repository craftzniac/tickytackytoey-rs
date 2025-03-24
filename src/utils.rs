use std::fmt;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum PlayerSlotValue {
    PlayerO = SlotValue::PlayerO as isize,
    PlayerX = SlotValue::PlayerX as isize,
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
