use core::renderer::Renderer;
use core::sprite::Sprite;
use core::unit;
use core::unit::{Vec2, AABB, Point, Unit, ToUnit, MS};

static PLAYER_MOVE_SPEED: Unit = Unit(0.3);

pub struct Player {
    score: int,

    sprite: Sprite,

    pos: Vec2,
    vy: Unit,

    aabb: AABB,
}

impl Player {
    pub fn new(renderer: &mut Renderer) -> Player {
        let mut sprite = Sprite::new(renderer, "assets/player.png".to_owned());
        let size = sprite.size();
        sprite.set_pivot(Point::new(size.w / 2, size.h / 2));

        Player {
            score: 0,

            sprite: sprite,

            pos: unit::vec2::ZERO,
            vy: Unit(0.0),

            aabb: AABB::new(
                unit::ZERO, unit::ZERO, size.w.to_unit(), size.h.to_unit()
            ),
        }
    }

    pub fn aabb(&self) -> AABB {
        self.aabb.transform(self.pos)
    }

    pub fn position(&self) -> Vec2 {
        self.pos
    }

    pub fn score(&self) -> int {
        self.score
    }

    pub fn offset(&mut self, offset: Vec2) {
        self.pos = self.pos + offset;
    }

    pub fn start_moving_up(&mut self) {
        self.vy = -PLAYER_MOVE_SPEED;
    }

    pub fn start_moving_down(&mut self) {
        self.vy = PLAYER_MOVE_SPEED;
    }

    pub fn stop_move(&mut self) {
        self.vy = Unit(0.0);
    }

    pub fn win(&mut self) {
        self.score += 1;
    }

    pub fn update(&mut self, dt: MS,
                  top_wall_aabb: &AABB,
                  bottom_wall_aabb: &AABB) {
        let dy = self.vy * dt;
        let aabb = self.aabb.transform(self.pos + Vec2::new(0.0, dy));
        if dy < unit::ZERO {
            if aabb.is_collided_with(top_wall_aabb) {
                self.pos.y = top_wall_aabb.bottom() + self.aabb.size().y / 2.0;
            } else {
                self.pos.y = self.pos.y + dy;
            }
        } else {
            if aabb.is_collided_with(bottom_wall_aabb) {
                self.pos.y = bottom_wall_aabb.top() - self.aabb.size().y / 2.0;
            } else {
                self.pos.y = self.pos.y + dy;
            }
        }
    }

    pub fn render(&self, renderer: &Renderer) {
        self.sprite.render(renderer, self.pos);
    }
}
