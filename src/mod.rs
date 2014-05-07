mod player;
mod ball;
pub mod scene;
/*
static FRAME_TIME: MS = MS(1000 / 60);
static MAX_TIME_BETWEEN_FRAME: MS = MS(1000);

pub fn run() {
    sdl2::init(sdl2::InitEverything);

    sdl2_image::init(sdl2_image::InitPng | sdl2_image::InitJpg);

    sdl2_mixer::init(sdl2_mixer::InitOgg);
    sdl2_mixer::open_audio(sdl2_mixer::DEFAULT_FREQUENCY, 0x8010u16, 2, 1024).unwrap();
    sdl2_mixer::allocate_channels(0);

    Pong::new().event_loop();

    sdl2_image::quit();
    sdl2::quit();
}

struct Pong {
    renderer: Renderer,
    input: Input,
    mixer: Mixer,
    scene_manager: SceneManager,
}

impl Pong {
    fn new() -> Pong {
        Pong {
            renderer: Renderer::new("pong-rs", Size::new(480, 320)),
            input: Input::new(),
            mixer: Mixer::new(),
            scene_manager: SceneManager::new(),
        }
    }

    fn event_loop(&mut self) {
        self.scene_manager.trans(~Main::new(&mut self.renderer, &mut self.mixer));

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
            let dt = current_time - last_update_time;
            if dt < MAX_TIME_BETWEEN_FRAME {
                self.update(dt);
            }
            last_update_time = current_time;

            self.render();

            let passed_time = MS(sdl2::timer::get_ticks()) - frame_start_time;
            if FRAME_TIME > passed_time {
                sdl2::timer::delay((FRAME_TIME - passed_time).val());
            }
        }
    }

    fn update(&mut self, dt: MS) {
        self.scene_manager.update(&self.input, dt);
    }

    fn render(&self) {
        self.scene_manager.render(&self.renderer);
    }
}
*/
