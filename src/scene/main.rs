use core::font::pixels::RGBA;
use core::input::keycode;
use core::scene::Scene;
use core::sprite::Sprite;
use core::font::Font;
use core::unit;
use core::unit::{Vec2, AABB};
use core::Core;

use player::Player;
use ball::Ball;

static PLAYER_PADDING: f32 = 16.0;

pub struct Main {
    background: Sprite,

    player1: Player,
    player2: Player,
    ball: Ball,

    font: Font,

    top_wall_aabb: AABB,
    bottom_wall_aabb: AABB,
    left_wall_aabb: AABB,
    right_wall_aabb: AABB,
}

impl Main {
    pub fn new(core: &mut Core) -> Main {
        let ref mut renderer = core.renderer;
        let ref mut mixer = core.mixer;

        let bg = Sprite::from_file(renderer, "assets/background.png".to_owned());

        let mut player1 = Player::new(renderer);
        player1.offset(Vec2::new(PLAYER_PADDING, 160.0));

        let mut player2 = Player::new(renderer);
        player2.offset(Vec2::new(480.0 - PLAYER_PADDING, 160.0));

        let mut ball = Ball::new(renderer, mixer);
        ball.reset();

        let font = Font::from_file(renderer, "assets/font.TTF");

        Main {
            background: bg,
            player1: player1,
            player2: player2,
            ball: ball,

            font: font,

            // set up AABB for walls according to `background.png`
            top_wall_aabb: AABB::new(240.0, 8.0, 480.0, 16.0),
            bottom_wall_aabb: AABB::new(240.0, 320.0 - 8.0, 480.0, 16.0),
            left_wall_aabb: AABB::new(-8.0, 160.0, 16.0, 320.0),
            right_wall_aabb: AABB::new(480.0 + 8.0, 160.0, 16.0, 320.0),
        }
    }

}

impl Scene for Main {
    fn update(&mut self, core: &mut Core) {
        let ref mut input = core.input;
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

        self.player1.update(core.dt, &self.top_wall_aabb, &self.bottom_wall_aabb);
        self.player2.update(core.dt, &self.top_wall_aabb, &self.bottom_wall_aabb);
        self.ball.update(core.dt,
                         &self.top_wall_aabb, &self.bottom_wall_aabb,
                         &self.left_wall_aabb, &self.right_wall_aabb,
                         &mut self.player1, &mut self.player2);
    }

    fn render(&mut self, core: &mut Core) {
        let ref renderer = core.renderer;

        renderer.clear();

        self.background.render(renderer, unit::vec2::ZERO);

        self.font.render(renderer, "NBX", RGBA(0, 255, 0, 255), Vec2::new(0.0, 32.0));

        self.player1.render(renderer);
        self.player2.render(renderer);
        self.ball.render(renderer);

        renderer.swap();
    }
}
