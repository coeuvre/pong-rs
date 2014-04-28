#![crate_type = "bin"]

extern crate collections;
extern crate native;

extern crate sdl2;
extern crate sdl2_image;
//extern crate sdl2_ttf;
extern crate sdl2_mixer;

mod sprite;
mod input;
mod unit;
mod renderer;
//mod timer;

mod pong;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    pong::run();
}
