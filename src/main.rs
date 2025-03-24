mod board;
mod game;
mod player;
mod utils;

fn main() {
    let game = game::Game::new();
    game.play();
}
