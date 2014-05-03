#![crate_type = "bin"]

extern crate collections;
extern crate rand;
extern crate native;

extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_mixer;
//extern crate sdl2_ttf;

mod sprite;
mod input;
mod unit;
mod renderer;
mod mixer;
mod scene;
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
