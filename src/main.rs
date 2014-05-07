#![crate_type = "bin"]

extern crate collections;
extern crate rand;
extern crate native;

extern crate core;

use core::Core;
use core::unit::Size;

mod player;
mod ball;
mod scene;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    let mut core = Core::new("pong-rs", Size::new(480, 320));
    let scene = ~scene::Main::new(&mut core);
    core.run(scene);
}
