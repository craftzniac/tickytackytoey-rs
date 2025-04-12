#![allow(dead_code)]

use crate::board::Board;
use crate::player::Player;
use crate::utils::{PlayerSlotValue, SlotUpdateStatus, WinState};

pub(crate) struct Game {
    player_o: Player,
    player_x: Player,
    board: Board,
    winner: Option<Player>,
    player_turn: Player,
}

impl Game {
    pub(crate) fn new() -> Self {
        let player_o = Player::new("john".into(), PlayerSlotValue::PlayerO);
        Self {
            player_o: player_o.clone(),
            player_x: Player::new("mary".into(), PlayerSlotValue::PlayerX),
            player_turn: player_o,
            winner: None,
            board: Board::new(),
        }
    }

    pub fn winner_or_draw(&mut self) -> WinState {
        // check the board to see if there's a draw, or a winner
        let result = self.board.check_match();

        match result {
            WinState::PlayerX => self.winner = Some(self.player_x.clone()),
            WinState::PlayerO => self.winner = Some(self.player_o.clone()),
            _ => {}
        }

        result
    }

    pub fn print_board(&self) {
        println!("{}", self.board);
    }

    fn switch_player_turn(&mut self) {
        match self.player_turn.slot_value {
            PlayerSlotValue::PlayerO => {
                self.player_turn = self.player_x.clone();
            }
            PlayerSlotValue::PlayerX => {
                self.player_turn = self.player_o.clone();
            }
        }
    }

    fn play_player_turn(&mut self) {
        let player_move = self.player_turn.get_move();
        // check if move is already played
        let update_status = self.board.update(player_move, &self.player_turn.slot_value);
        if update_status == SlotUpdateStatus::Duplicate {
            println!("{}", update_status);
            self.play_player_turn();
        }
    }

    pub(crate) fn play(&mut self) {
        self.print_board();
        // game loop repeats as long as there isn't a winner and there's no draw
        while let WinState::None = self.winner_or_draw() {
            self.switch_player_turn();
            self.play_player_turn();
            self.print_board();
        }

        match self.winner_or_draw() {
            WinState::Draw => println!("It's a draw!"),
            WinState::PlayerO | WinState::PlayerX => {
                // print the winner
                println!("Player {} has won this round", self.winner.clone().unwrap());
            }
            _ => {
                panic!("This was never supposed to run");
            }
        }
    }
}
