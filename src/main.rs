mod board;
mod game;
mod player;
mod utils;

fn main() {
    let mut game = game::Game::new();
    game.play();
}
