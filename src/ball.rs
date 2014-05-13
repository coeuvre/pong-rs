use rand;

use std::rc::Rc;

use game::sprite::Sprite;
use game::renderer::Renderer;
use game::mixer::{Mixer, Music};
use game::unit;
use game::unit::{Unit, Vec2, AABB, Point, MS};

use super::player::Player;

static BALL_MOVE_SPEED: f32 = 0.5;

pub struct Ball {
    sprite: Sprite,

    pos: Vec2,
    v: Vec2,

    aabb: AABB,

    beeep: Rc<Music>,
    peeeeeep: Rc<Music>,
    plop: Rc<Music>,
}

impl Ball {
    pub fn new(renderer: &mut Renderer, mixer: &mut Mixer) -> Ball {
        let mut sprite = Sprite::from_file(renderer, "assets/ball.png".to_owned());
        let size = sprite.size();
        sprite.set_pivot(Point::new(size.w / 2, size.h / 2));

        Ball {
            sprite: sprite,
            pos: unit::vec2::ZERO,
            v: unit::vec2::ZERO,
            aabb: AABB::new(
                0.0, 0.0, size.w, size.h
            ),

            beeep: mixer.load_music("assets/beeep.ogg".to_owned()),
            peeeeeep: mixer.load_music("assets/peeeeeep.ogg".to_owned()),
            plop: mixer.load_music("assets/plop.ogg".to_owned()),
        }
    }

    pub fn emit(&mut self) {
        if self.v.length() == unit::ZERO {
            let dx = if rand::random::<f32>() > 0.5 {
                0.5
            } else {
                -0.5
            };
            self.set_direction(Vec2::new(dx,
                                         rand::random::<f32>() * 2.0 - 1.0));
        }
    }

    pub fn update(&mut self, dt: MS,
                  top_wall_aabb: &AABB, bottom_wall_aabb: &AABB,
                  left_wall_aabb: &AABB, right_wall_aabb: &AABB,
                  player1: &mut Player, player2: &mut Player) {
        let dp = self.v * dt;

        // check collision of x direction
        let aabb = self.aabb.transform(self.pos + Vec2::new(dp.x, 0.0));
        if dp.x < unit::ZERO {
            if aabb.is_collided_with(left_wall_aabb) {
                player2.win();
                println!("{} : {}", player1.score(), player2.score());
                self.reset();

                self.peeeeeep.play(1).unwrap();
                return;
            } else if aabb.is_collided_with(&player1.aabb()) {
                let dy = self.reflection(player1);
                self.set_direction(Vec2::new(1.0, dy));

                self.beeep.play(1).unwrap();
            } else {
                self.pos.x = self.pos.x + dp.x;
            }
        } else {
            if aabb.is_collided_with(right_wall_aabb) {
                player1.win();
                println!("{} : {}", player1.score(), player2.score());
                self.reset();

                self.peeeeeep.play(1).unwrap();
                return;
            } else if aabb.is_collided_with(&player2.aabb()) {
                let dy = self.reflection(player2);
                self.set_direction(Vec2::new(-1.0, dy));

                self.beeep.play(1).unwrap();
            } else {
                self.pos.x = self.pos.x + dp.x;
            }
        }

        // check collision of y direction
        let aabb = self.aabb.transform(self.pos + Vec2::new(0.0, dp.y));
        if dp.y < unit::ZERO {
            if aabb.is_collided_with(top_wall_aabb) {
                self.pos.y = top_wall_aabb.bottom() + self.aabb.size().y / 2.0;
                self.v.y = -self.v.y;

                self.plop.play(1).unwrap();
            } else {
                self.pos.y = self.pos.y + dp.y;
            }
        } else {
            if aabb.is_collided_with(bottom_wall_aabb) {
                self.pos.y = bottom_wall_aabb.top() - self.aabb.size().y / 2.0;
                self.v.y = -self.v.y;

                self.plop.play(1).unwrap();
            } else {
                self.pos.y = self.pos.y + dp.y;
            }
        }
    }

    pub fn render(&self, renderer: &Renderer) {
        self.sprite.render(renderer, self.pos);
    }

    pub fn reset(&mut self) {
        self.v = unit::vec2::ZERO;
        self.pos = Vec2::new(240.0, 160.0);
    }

    fn set_direction(&mut self, dir: Vec2) {
        self.v = dir.norm() * BALL_MOVE_SPEED;
    }

    fn reflection(&self, player: &Player) -> Unit {
        let mut y = self.pos.y - player.position().y;

        y = y * rand::random::<f32>() * 0.4 + 0.8;

        if y > player.aabb().size.y / 2.0 {
            y = player.aabb().size.y / 2.0;
        } else if y < -player.aabb().size.y / 2.0 {
            y = -player.aabb().size.y / 2.0;
        }

        y * 4.0 / player.aabb().size.y
    }
}
