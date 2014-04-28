use sprite::Sprite;
use renderer::Renderer;

use unit;
use unit::{Unit, Vec2, AABB, Point, ToUnit, MS};

pub struct Ball {
    sprite: Sprite,

    pos: Vec2,
    v: Vec2,

    aabb: AABB,
}

impl Ball {
    pub fn new(renderer: &mut Renderer) -> Ball {
        let mut sprite = Sprite::new(renderer, ~"assets/ball.png");
        let size = sprite.size();
        sprite.set_pivot(Point::new(size.w / 2, size.h / 2));

        Ball {
            sprite: sprite,
            pos: unit::vec2::ZERO,
            v: unit::vec2::ZERO,
            aabb: AABB::new(
                unit::ZERO, unit::ZERO, size.w.to_unit(), size.h.to_unit()
            ),
        }
    }

    pub fn emit(&mut self) {
        if self.v.length() == unit::ZERO {
            self.v = Vec2::new(Unit(-0.1), Unit(0.24));
        }
    }

    pub fn offset(&mut self, offset: Vec2) {
        self.pos = self.pos + offset;
    }

    pub fn update(&mut self, dt: MS, top_wall_aabb: &AABB, bottom_wall_aabb: &AABB,
                  left_wall_aabb: &AABB, right_wall_aabb: &AABB) {
        let dp = self.v * dt;

        // check collision of y direction
        let aabb = self.aabb.transform(self.pos + Vec2::new(unit::ZERO, dp.y));
        if dp.y < unit::ZERO {
            if aabb.is_collided_with(top_wall_aabb) {
                self.pos.y = top_wall_aabb.bottom() + self.aabb.size().y / 2.0;
                self.v.y = -self.v.y;
            } else {
                self.pos.y = self.pos.y + dp.y;
            }
        } else {
            if aabb.is_collided_with(bottom_wall_aabb) {
                self.pos.y = bottom_wall_aabb.top() - self.aabb.size().y / 2.0;
                self.v.y = -self.v.y;
            } else {
                self.pos.y = self.pos.y + dp.y;
            }
        }

        // check collision of x direction
        let aabb = self.aabb.transform(self.pos + Vec2::new(dp.x, unit::ZERO));
        if dp.x < unit::ZERO {
            if aabb.is_collided_with(left_wall_aabb) {
                self.pos.x = left_wall_aabb.right() + self.aabb.size().x / 2.0;
                self.v.x = -self.v.x;
            } else {
                self.pos.x = self.pos.x + dp.x;
            }
        } else {
            if aabb.is_collided_with(right_wall_aabb) {
                self.v.x = -self.v.x;
                self.pos.x = right_wall_aabb.left() - self.aabb.size().x / 2.0;
            } else {
                self.pos.x = self.pos.x + dp.x;
            }
        }
    }

    pub fn render(&self, renderer: &Renderer) {
        self.sprite.render(renderer, self.pos);
    }
}
