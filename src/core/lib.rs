#![crate_type = "lib"]
#![crate_id="core#core:0.1"]

extern crate collections;

extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_mixer;

use std::rc::Rc;
use std::cell::RefCell;

use sdl2::{event, keycode};

use renderer::Renderer;
use mixer::Mixer;
use input::Input;
use scene::{Scene, SceneManager};
use unit::{Size, MS};

pub mod sprite;
pub mod input;
pub mod unit;
pub mod renderer;
pub mod mixer;
pub mod scene;
pub mod timer;

static DEFAULT_FRAME_TIME: MS = MS(1000 / 30);
static MAX_TIME_BETWEEN_FRAME: MS = MS(1000);

pub struct Core {
    pub renderer: Renderer,
    pub input: Input,
    pub mixer: Mixer,
    pub dt: MS,

    scene_manager: Rc<RefCell<SceneManager>>,
    frame_time: MS,
}

impl Core {
    pub fn new(title: &str, size:Size) -> Core {
        sdl2::init(sdl2::InitEverything);

        sdl2_image::init(sdl2_image::InitPng | sdl2_image::InitJpg);

        sdl2_mixer::init(sdl2_mixer::InitOgg);
        sdl2_mixer::open_audio(sdl2_mixer::DEFAULT_FREQUENCY, 0x8010u16, 2, 1024).unwrap();
        sdl2_mixer::allocate_channels(0);

        Core {
            renderer: Renderer::new(title, size),
            input: Input::new(),
            mixer: Mixer::new(),
            scene_manager: Rc::<RefCell<SceneManager>>::new(RefCell::<SceneManager>::new(SceneManager::new())),

            frame_time: DEFAULT_FRAME_TIME,
            dt: MS(0),
        }
    }

    pub fn run(&mut self, scene: ~Scene) {
        self.scene_manager.borrow_mut().trans(scene);

        let mut is_running = true;
        let mut last_update_time = MS(sdl2::timer::get_ticks());

        while is_running {
            let frame_start_time = MS(sdl2::timer::get_ticks());

            self.input.begin_new_frame();

            match event::poll_event() {
                event::KeyDownEvent(_, _, keycode, _, _) => {
                    self.input.key_down_event(keycode);
                },
                event::KeyUpEvent(_, _, keycode, _, _) => {
                    self.input.key_up_event(keycode);
                },
                event::QuitEvent(_) => is_running = false,
                _ => {},
            }

            if self.input.is_key_pressed(keycode::EscapeKey) {
                is_running = false;
            }

            let current_time = MS(sdl2::timer::get_ticks());
            self.dt = current_time - last_update_time;
            if self.dt < MAX_TIME_BETWEEN_FRAME {
                self.update();
            }
            last_update_time = current_time;

            self.render();

            let passed_time = MS(sdl2::timer::get_ticks()) - frame_start_time;
            if self.frame_time > passed_time {
                sdl2::timer::delay((self.frame_time - passed_time).val());
            }
        }
    }

    fn update(&mut self) {
        self.scene_manager.clone().borrow_mut().update(self);
    }

    fn render(&mut self) {
        self.scene_manager.clone().borrow_mut().render(self);
    }
}
