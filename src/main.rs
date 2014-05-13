#![crate_type = "bin"]

extern crate collections;
extern crate rand;
extern crate native;

extern crate game;

use game::Game;
use game::unit::Size;

mod player;
mod ball;
mod scene;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    let mut game = Game::new("pong-rs", Size::new(480, 320));
    let scene = box scene::Main::new(&mut game);
    game.run(scene);
}
