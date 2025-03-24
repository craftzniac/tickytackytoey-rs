use crate::utils::SlotValue;

pub(crate) struct Player {
    name: String,
    slot_value: SlotValue,
}

impl Player {
    pub fn new(name: String, slot_value: SlotValue) -> Self {
        Self { name, slot_value }
    }

    fn play_turn(&self) {
        todo!("implemnet play_turn")
    }
}
