use sdl2;
use sdl2::{event, keycode};

use sdl2_image;
use sdl2_mixer;

use input::Input;
use renderer;
use sprite::Sprite;
use mixer::Mixer;

use unit;
use unit::{Size, MS, Vec2, AABB};

use self::player::Player;
use self::ball::Ball;

mod player;
mod ball;

static FRAME_TIME: MS = MS(1000 / 60);
static MAX_TIME_BETWEEN_FRAME: MS = MS(1000);

static PLAYER_PADDING: f32 = 16.0;

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
    renderer: renderer::Renderer,
    background: Sprite,

    player1: Player,
    player2: Player,
    ball: Ball,

    top_wall_aabb: AABB,
    bottom_wall_aabb: AABB,
    left_wall_aabb: AABB,
    right_wall_aabb: AABB,
}

impl Pong {
    fn new() -> Pong {
        let mut mixer = Mixer::new();

        let mut renderer = renderer::Renderer::new(Size::new(480, 320));
        let bg = Sprite::new(&mut renderer, ~"assets/background.png");

        let mut player1 = Player::new(&mut renderer);
        player1.offset(Vec2::new(PLAYER_PADDING, 160.0));

        let mut player2 = Player::new(&mut renderer);
        player2.offset(Vec2::new(480.0 - PLAYER_PADDING, 160.0));

        let mut ball = Ball::new(&mut renderer, &mut mixer);
        ball.reset();

        Pong {
            renderer: renderer,
            background: bg,
            player1: player1,
            player2: player2,
            ball: ball,

            // set up AABB for walls according to `background.png`
            top_wall_aabb: AABB::new(240.0, 8.0, 480.0, 16.0),
            bottom_wall_aabb: AABB::new(240.0, 320.0 - 8.0, 480.0, 16.0),
            left_wall_aabb: AABB::new(-8.0, 160.0, 16.0, 320.0),
            right_wall_aabb: AABB::new(480.0 + 8.0, 160.0, 16.0, 320.0),
        }
    }

    fn event_loop(&mut self) {
        let mut is_running = true;
        let mut last_update_time = MS(sdl2::timer::get_ticks());
        let mut input = Input::new();

        while is_running {
            let frame_start_time = MS(sdl2::timer::get_ticks());

            input.begin_new_frame();

            match event::poll_event() {
                event::KeyDownEvent(_, _, keycode, _, _) => {
                    input.key_down_event(keycode);
                },
                event::KeyUpEvent(_, _, keycode, _, _) => {
                    input.key_up_event(keycode);
                },
                event::QuitEvent(_) => is_running = false,
                _ => {},
            }

            if input.is_key_pressed(keycode::EscapeKey) {
                is_running = false;
            }

            // player1 input
            if input.is_key_held(keycode::WKey) && input.is_key_held(keycode::SKey) {
                self.player1.stop_move();
            } else if input.is_key_held(keycode::WKey) {
                self.player1.start_moving_up();
            } else if input.is_key_held(keycode::SKey) {
                self.player1.start_moving_down();
            } else {
                self.player1.stop_move();
            }

            // player2 input
            if input.is_key_held(keycode::UpKey) &&
               input.is_key_held(keycode::DownKey) {
                self.player2.stop_move();
            } else if input.is_key_held(keycode::UpKey) {
                self.player2.start_moving_up();
            } else if input.is_key_held(keycode::DownKey) {
                self.player2.start_moving_down();
            } else {
                self.player2.stop_move();
            }

            // emit ball
            if input.is_key_pressed(keycode::SpaceKey) {
                self.ball.emit();
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
        self.player1.update(dt, &self.top_wall_aabb, &self.bottom_wall_aabb);
        self.player2.update(dt, &self.top_wall_aabb, &self.bottom_wall_aabb);
        self.ball.update(dt,
                         &self.top_wall_aabb, &self.bottom_wall_aabb,
                         &self.left_wall_aabb, &self.right_wall_aabb,
                         &mut self.player1, &mut self.player2);
    }

    fn render(&self) {
        self.renderer.clear();

        self.background.render(&self.renderer, unit::vec2::ZERO);

        self.player1.render(&self.renderer);
        self.player2.render(&self.renderer);
        self.ball.render(&self.renderer);

        self.renderer.swap();
    }
}
