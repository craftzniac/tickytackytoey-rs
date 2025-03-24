use crate::board::Board;
use crate::player::Player;
use crate::utils::SlotValue;

pub(crate) struct Game {
    player_o: Player,
    player_x: Player,
    board: Board,
}

impl Game {
    pub(crate) fn new() -> Self {
        Self {
            player_o: Player::new("john".into(), SlotValue::PlayerO),
            player_x: Player::new("john".into(), SlotValue::PlayerX),
            board: Board::new(),
        }
    }

    pub(crate) fn play(&self) {
        println!("{}", self.board);
    }
}
